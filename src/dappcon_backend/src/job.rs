pub mod distribution;
mod generators;
use std::fmt;

use ethers_core::types::{Address, U256};
use ic_cdk::{api::management_canister::main::raw_rand, println};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

use crate::{
    evm_rpc::LogEntry,
    job::generators::{generate_and_store_image, generate_and_store_metadata, generate_attributes},
    state::{mutate_state, LogSource},
};
use std::str::FromStr;

pub async fn job(event_source: LogSource, event: LogEntry) {
    mutate_state(|s| s.record_processed_log(event_source.clone()));
    let mint_event = MintEvent::from(event);
    println!("{:?}", &mint_event);
    let random_bytes = get_random_bytes().await;
    let mut rng = ChaCha20Rng::from_seed(random_bytes);
    let attributes = generate_attributes(&mut rng);
    generate_and_store_metadata(&mint_event, &attributes);
    generate_and_store_image(&mint_event, &attributes);
}

async fn get_random_bytes() -> [u8; 32] {
    let (raw_rand,): (Vec<u8>,) = raw_rand()
        .await
        // TODO: make sure its safe to trap here
        .unwrap_or_else(|_e| ic_cdk::trap("call to raw_rand failed"));
    let raw_rand_32_bytes: [u8; 32] = raw_rand
        .try_into()
        .unwrap_or_else(|_e| panic!("raw_rand not 32 bytes"));
    raw_rand_32_bytes
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
