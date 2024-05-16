use rand::distributions::{Distribution, WeightedIndex};
use rand_chacha::ChaCha20Rng;
use serde_json::{json, to_vec};
use svg::{
    node::element::{Circle, Rectangle},
    Document,
};

use crate::storage::{store_asset, Asset};

use super::MintEvent;

pub const SVG_COLOR_KEYWORDS: [&str; 4] = ["blue", "green", "yellow", "red"];

pub const WEIGHTS: [u32; 4] = [3, 3, 3, 1];

pub struct Attributes {
    pub bg_color: String,
    pub circle_color: String,
}

pub fn generate_attributes(rng: &mut ChaCha20Rng) -> Attributes {
    Attributes {
        bg_color: select_value(&SVG_COLOR_KEYWORDS, &WEIGHTS, rng).to_string(),
        circle_color: select_value(&SVG_COLOR_KEYWORDS, &WEIGHTS, rng).to_string(),
    }
}

fn select_value<T>(values: &[T], weights: &[u32], rng: &mut ChaCha20Rng) -> T
where
    T: Clone,
{
    let dist = WeightedIndex::new(weights).unwrap();
    values[dist.sample(rng)].clone()
}

pub fn generate_and_store_metadata(mint_event: &MintEvent, attributes: &Attributes) {
    let metadata = json!({
        "name" : format!("dappcon #{}", mint_event.token_id),
        "image" : format!("http://{}.localhost:4943/{}.svg",
          ic_cdk::id().to_text(),
          mint_event.token_id
        ),
        "attributes" : [
          {
            "trait_type" : "bg_color",
            "value" : attributes.bg_color
          },
          {
            "trait_type" : "circle_color",
            "value" : attributes.circle_color
          }
        ]
    });

    let byte_vec = to_vec(&metadata).expect("json should be serializblae to bytes");

    store_asset(
        format!("/{}", mint_event.token_id),
        Asset {
            headers: vec![("Content-Type".to_string(), "text/json".to_string())],
            body: byte_vec,
        },
    );
}

pub fn generate_and_store_image(mint_event: &MintEvent, attributes: &Attributes) {
    let bg = Rectangle::new()
        .set("width", "100%")
        .set("height", "100%")
        .set("fill", attributes.bg_color.clone());

    let circle = Circle::new()
        .set("cy", 50)
        .set("cx", 50)
        .set("r", 48)
        .set("fill", attributes.circle_color.clone());

    let document = Document::new()
        .set("viewBox", (0, 0, 100, 100))
        .add(bg)
        .add(circle);

    let byte_vec = document.to_string().into_bytes();

    store_asset(
        format!("/{}.svg", mint_event.token_id),
        Asset {
            headers: vec![("Content-Type".to_string(), "image/svg+xml".to_string())],
            body: byte_vec,
        },
    )
}
