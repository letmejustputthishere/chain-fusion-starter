mod calculate_result;
mod submit_result;

use std::fmt;

use ethers_core::types::U256;
use ic_cdk::println;
use submit_result::submit_result;

use crate::{
    evm_rpc::LogEntry,
    state::{mutate_state, LogSource},
};

pub async fn job(event_source: LogSource, event: LogEntry) {
    mutate_state(|s| s.record_processed_log(event_source.clone()));
    // because we deploy the canister with topics only matching
    // NewJob events we can safely assume that the event is a NewJob.

    //let new_job_event = NewJobEvent::from(event);

    // this is what the smart contract expects to know
    //let job_id = new_job_event.job_id;

    // todo: read sleep duration from the job

    // sleep 30s
    // let interval = std::time::Duration::from_secs(30);
    // let do_it_now = false;
    // ic_cdk::println!("Starting a periodic task with interval {interval:?}");
    // ic_cdk_timers::set_timer(interval, || {
    //     do_it_now = true;
    // });

    // loop {
    //     _ = &mut do_it_now => {
    //             // If there's another case where `do_it_now` is updated, handle it here
    //             // This is a placeholder to show how to handle other async events
    //             do_it_now = true;
    //         },
    //     if do_it_now == true {
    //         // do it now
    //         submit_result(U256::from(0)).await;
    //     }
    // }

    // // sleep
    // ic_cdk::println!("Sleeping for {interval:?}");
    // ic_cdk_timers::set_timer(interval, || {
    //     ic_cdk::println!("Waking up");
    // });

    // todo: pass job_id from log
    submit_result(U256::from(0)).await;
    println!("Successfully ran job #{:?}", 0);
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NewJobEvent {
    pub job_id: U256,
}

impl fmt::Debug for NewJobEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NewJobEvent")
            .field("job_id", &self.job_id)
            .finish()
    }
}

impl From<LogEntry> for NewJobEvent {
    fn from(entry: LogEntry) -> NewJobEvent {
        // we expect exactly 2 topics from the NewJob event.
        // you can read more about event signatures [here](https://docs.alchemy.com/docs/deep-dive-into-eth_getlogs#what-are-event-signatures)
        let job_id =
            U256::from_str_radix(&entry.topics[1], 16).expect("the token id should be valid");

        NewJobEvent { job_id }
    }
}
