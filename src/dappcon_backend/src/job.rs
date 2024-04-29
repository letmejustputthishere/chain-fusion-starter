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
    // because we deploy the canister with topics only matching
    // Transfer events with the from topic set to the zero address
    // we can safely assume that the event is a mint event.
    let mint_event = MintEvent::from(event);
    // we get secure random bytes from the IC to seed the RNG
    // for every mint event.
    // you can read more [here](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/advanced-features/randomness/)
    let random_bytes = get_random_bytes().await;
    let mut rng = ChaCha20Rng::from_seed(random_bytes);
    // using th random number generator seeded with the on-chain random bytes
    // we generate our attributes for the NFT
    let attributes = generate_attributes(&mut rng);
    // based on the attributes we generate and store the opensea compliant
    // metadata in the canisters stable memory.
    // canister can currently access 400GB of mutable on-chain storage, you can read more about
    // this feature [here](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/maintain/storage/).
    generate_and_store_metadata(&mint_event, &attributes);
    // last, based on the attributes we generate and store the image in the canisters stable memory.
    generate_and_store_image(&mint_event, &attributes);
    println!("Assets & Metadata successfully generated: http://2222s-4iaaa-aaaaf-ax2uq-cai.localhost:4943/{:?}", &mint_event.token_id);
}

// This function asynchronously retrieves a random byte array of length 32.
async fn get_random_bytes() -> [u8; 32] {
    // Call the `raw_rand` function and await its result.
    let (raw_rand,): (Vec<u8>,) = raw_rand()
        .await
        .unwrap_or_else(|_e| ic_cdk::trap("call to raw_rand failed"));

    // Convert the obtained byte vector into a fixed-size array of length 32.
    let raw_rand_32_bytes: [u8; 32] = raw_rand
        .try_into()
        .unwrap_or_else(|_e| panic!("raw_rand not 32 bytes"));

    // Return the resulting 32-byte array.
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
        // we expect exactly 4 topics from the transfer event.
        // you can read more about event signatures [here](https://docs.alchemy.com/docs/deep-dive-into-eth_getlogs#what-are-event-signatures)
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
