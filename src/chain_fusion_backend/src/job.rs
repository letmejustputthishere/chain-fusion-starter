mod generators;

use ethers_core::types::U256;
use ic_cdk::{api::management_canister::main::raw_rand, println};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

use crate::{
    evm_rpc::LogEntry,
    job::generators::generate_and_store_image,
    state::{mutate_state, LogSource},
};

use self::generators::{generate_and_store_metadata, generate_attributes};

pub async fn job(event_source: LogSource, event: LogEntry) {
    mutate_state(|s| s.record_processed_log(event_source.clone()));
    // because we deploy the canister with topics only matching
    // NewJob events we can safely assume that the event is a NewJob.
    let mint_event = MintEvent::from(event);
    let random_bytes = get_random_bytes().await;
    let mut rng = ChaCha20Rng::from_seed(random_bytes);
    let attributes = generate_attributes(&mut rng);
    generate_and_store_metadata(&mint_event, &attributes);
    generate_and_store_image(&mint_event, &attributes);
    println!(
        "token generate http://{}.localhost:4943/{}",
        ic_cdk::id().to_text(),
        mint_event.token_id
    )
}

async fn get_random_bytes() -> [u8; 32] {
    let (raw_rand,) = raw_rand().await.expect("calls to raw_rand should not fail");
    raw_rand
        .try_into()
        .expect("raw_rad should contain 32 bytes")
}

pub struct MintEvent {
    pub token_id: U256,
}

impl From<LogEntry> for MintEvent {
    fn from(entry: LogEntry) -> MintEvent {
        // we expect exactly 4 topics from the NewJob event.
        // you can read more about event signatures [here](https://docs.alchemy.com/docs/deep-dive-into-eth_getlogs#what-are-event-signatures)
        let token_id =
            U256::from_str_radix(&entry.topics[3], 16).expect("the token id should be valid");

        MintEvent { token_id }
    }
}
