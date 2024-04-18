use ic_cdk::println;

use crate::{
    evm_rpc::LogEntry,
    state::{mutate_state, LogSource},
    storage::store_asset,
};

pub async fn job(event_source: LogSource, event: LogEntry) -> Result<(), ()> {
    mutate_state(|s| s.record_processed_log(event_source.clone()));
    println!("stored asset under key {}", event_source.to_asset_key());
    store_asset(
        event_source.to_asset_key(),
        crate::storage::Asset {
            headers: vec![(String::from("Content-Type"), String::from("text/plain"))],
            body: event.data.into(),
        },
    );
    Ok(())
}
