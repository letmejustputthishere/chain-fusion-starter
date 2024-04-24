use std::fmt;

use ethers_core::types::{Address, U256};
use ic_cdk::println;
use serde_json::{json, to_vec};

use crate::{
    evm_rpc::LogEntry,
    state::{mutate_state, LogSource},
    storage::store_asset,
};
use std::str::FromStr;

pub async fn job(event_source: LogSource, event: LogEntry) -> Result<(), ()> {
    mutate_state(|s| s.record_processed_log(event_source.clone()));
    let mint_event = MintEvent::from(event);
    println!("{:?}", &mint_event);
    // create JSON metadata with serde_json
    let metadata = json!({
        "name": format!("Motoko #{}", mint_event.token_id),
        "image": format!("https://{}.raw.icp0.io/{}.png",ic_cdk::id().to_text(), &mint_event.token_id),
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
    Ok(())
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MintEvent {
    pub from_address: Address,
    pub to_address: Address,
    pub token_id: U256,
}

impl fmt::Debug for MintEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MintEvent")
            .field("from_address", &self.from_address)
            .field("to_address", &self.to_address)
            .field("token_id", &self.token_id)
            .finish()
    }
}

impl From<LogEntry> for MintEvent {
    fn from(entry: LogEntry) -> MintEvent {
        // we expect exactly 4 topics from the transfer event
        let from_address =
            ethers_core::types::Address::from_str(&entry.topics[1][entry.topics[1].len() - 40..])
                .expect("the address contained in the first topic should be valid");
        let to_address =
            ethers_core::types::Address::from_str(&entry.topics[2][entry.topics[1].len() - 40..])
                .expect("the address contained in the second topic should be valid");
        let token_id =
            U256::from_str_radix(&entry.topics[3], 16).expect("the token id should be valid");

        MintEvent {
            from_address,
            to_address,
            token_id,
        }
    }
}
