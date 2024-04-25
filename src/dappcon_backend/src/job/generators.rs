use super::distribution::SVG_COLOR_KEYWORDS;
use super::distribution::WEIGHTS;
use super::MintEvent;
use crate::{evm_rpc::RpcServices, storage::store_asset};
use rand::distributions::{Distribution, WeightedIndex};
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_vec};
use svg::node::element::Circle;
use svg::node::element::Rectangle;
use svg::Document;

#[derive(Serialize)]
pub struct Attributes {
    pub bg_color: String,
    pub frame_color: String,
    pub circle_color: String,
}

#[derive(Serialize, Deserialize)]
struct Trait {
    pub trait_type: String,
    pub value: String,
}

impl Attributes {
    fn to_trait_list(&self) -> Vec<Trait> {
        serde_json::to_value(self)
            .unwrap()
            .as_object()
            .unwrap()
            .clone()
            .into_iter()
            .map(|(k, v)| Trait {
                trait_type: k,
                value: v.as_str().unwrap().to_string(),
            })
            .collect()
    }
}

fn select_value<T>(values: &[T], weights: &[u32], rng: &mut ChaCha20Rng) -> T
where
    T: Clone,
{
    let dist = WeightedIndex::new(weights).unwrap();
    values[dist.sample(rng)].clone()
}

pub fn generate_attributes(rng: &mut ChaCha20Rng) -> Attributes {
    Attributes {
        bg_color: select_value(&SVG_COLOR_KEYWORDS, &WEIGHTS, rng).to_string(),
        frame_color: select_value(&SVG_COLOR_KEYWORDS, &WEIGHTS, rng).to_string(),
        circle_color: select_value(&SVG_COLOR_KEYWORDS, &WEIGHTS, rng).to_string(),
    }
}

pub fn generate_and_store_image(mint_event: &MintEvent, attributes: &Attributes) {
    let bg = Rectangle::new()
        .set("width", "100%")
        .set("height", "100%")
        .set("fill", attributes.bg_color.clone());

    let frame = Rectangle::new()
        .set("x", 1)
        .set("y", 1)
        .set("width", 98)
        .set("height", 98)
        .set("fill", "none")
        .set("stroke", attributes.frame_color.clone())
        .set("stroke-width", 2);

    let circle = Circle::new()
        .set("cx", 50)
        .set("cy", 50)
        .set("r", 48)
        .set("fill", attributes.circle_color.clone());

    let document = Document::new()
        .set("viewBox", (0, 0, 100, 100))
        .add(bg)
        .add(frame)
        .add(circle);

    // Serialize the SVG document to a Vec<u8>
    let byte_vec = document.to_string().into_bytes();
    store_asset(
        format!("/{}.svg", mint_event.token_id),
        crate::storage::Asset {
            headers: vec![(String::from("Content-Type"), String::from("image/svg+xml"))],
            body: byte_vec,
        },
    );
}

pub fn generate_and_store_metadata(mint_event: &MintEvent, attributes: &Attributes) {
    // get RpcService from state
    let rpc_services = crate::state::read_state(|s| s.rpc_services.clone());
    let is_local_rpc = match rpc_services {
        RpcServices::Custom {
            chainId: _,
            services,
        } => {
            let mut found_local = false;
            for service in services {
                if service.url.contains("localhost") {
                    found_local = true;
                    break;
                }
            }
            found_local
        }
        _ => false,
    };
    let image_url = if is_local_rpc {
        format!(
            "http://{}.localhost:4943/{}.svg",
            ic_cdk::id().to_text(),
            &mint_event.token_id
        )
    } else {
        format!(
            "https://{}.raw.icp0.io/{}.svg",
            ic_cdk::id().to_text(),
            &mint_event.token_id
        )
    };
    // create JSON metadata with serde_json
    let metadata = json!({
        "name": format!("Chainfusion #{}", mint_event.token_id),
        "image": image_url,
        "attributes" : attributes.to_trait_list(),
    });
    // Serialize the JSON value to a Vec<u8>
    let byte_vec: Vec<u8> = match to_vec(&metadata) {
        Ok(vec) => vec,
        Err(_) => {
            ic_cdk::trap("Failed to serialize JSON");
        }
    };
    store_asset(
        format!("/{}", mint_event.token_id),
        crate::storage::Asset {
            headers: vec![(String::from("Content-Type"), String::from("text/json"))],
            body: byte_vec,
        },
    );
}
