use std::{
    cmp::{min, Ordering},
    ops::{Add, Div, Sub},
    time::Duration,
};

use candid::Nat;
use ic_cdk::println;

use crate::{
    evm_rpc::{
        BlockTag, GetBlockByNumberResult, GetLogsArgs, GetLogsResult, HttpOutcallError,
        MultiGetBlockByNumberResult, MultiGetLogsResult, RejectionCode, RpcError, EVM_RPC,
    },
    guard::TimerGuard,
    job::job,
    state::{mutate_state, read_state, State, TaskType},
};

async fn process_logs() {
    let _guard = match TimerGuard::new(TaskType::ProcessLogs) {
        Ok(guard) => guard,
        Err(_) => return,
    };

    let logs_to_process = read_state(|s| (s.logs_to_process.clone()));

    for (event_source, event) in logs_to_process {
        println!("running job");
        job(event_source, event).await
    }
}

pub async fn get_logs(from: &Nat, to: &Nat) -> GetLogsResult {
    let get_logs_address = read_state(|s| s.get_logs_address.clone());
    let get_logs_topics = read_state(|s| s.get_logs_topics.clone());
    let rpc_services = read_state(|s| s.rpc_services.clone());
    let get_logs_args: GetLogsArgs = GetLogsArgs {
        fromBlock: Some(BlockTag::Number(from.clone())),
        toBlock: Some(BlockTag::Number(to.clone())),
        addresses: get_logs_address.to_vec(),
        topics: get_logs_topics.clone(),
    };

    let cycles = 10_000_000_000;
    let (result,) = EVM_RPC
        .eth_get_logs(rpc_services, None, get_logs_args, cycles)
        .await
        .expect("Call failed");

    match result {
        MultiGetLogsResult::Consistent(r) => r,
        MultiGetLogsResult::Inconsistent(_) => {
            panic!("RPC providers gave inconsistent results")
        }
    }
}

/// Scraps Ethereum logs between `from` and `min(from + MAX_BLOCK_SPREAD, to)` since certain RPC providers
/// require that the number of blocks queried is no greater than MAX_BLOCK_SPREAD.
/// Returns the last block number that was scraped (which is `min(from + MAX_BLOCK_SPREAD, to)`) if there
/// was no error when querying the providers, otherwise returns `None`.
async fn scrape_eth_logs_range_inclusive(from: &Nat, to: &Nat) -> Option<Nat> {
    /// The maximum block spread is introduced by Alchemy limits.
    const MAX_BLOCK_SPREAD: u16 = 500;
    match from.cmp(to) {
        Ordering::Less | Ordering::Equal => {
            let max_to = from.clone().add(Nat::from(MAX_BLOCK_SPREAD));
            let mut last_block_number = min(max_to, to.clone());
            println!(
                "Scraping ETH logs from block {:?} to block {:?}...",
                from, last_block_number
            );

            let logs = loop {
                match get_logs(from, &last_block_number).await {
                    GetLogsResult::Ok(logs) => break logs,
                    GetLogsResult::Err(e) => {
                        println!(
                          "Failed to get ETH logs from block {from} to block {last_block_number}: {e:?}",
                      );
                        match e {
                            RpcError::HttpOutcallError(e) => {
                                if e.is_response_too_large() {
                                    if *from == last_block_number {
                                        mutate_state(|s| {
                                            s.record_skipped_block(last_block_number.clone());
                                            s.last_scraped_block_number = last_block_number.clone();
                                        });
                                        return Some(last_block_number);
                                    } else {
                                        let new_last_block_number = from.clone().add(
                                            last_block_number
                                                .clone()
                                                .sub(from.clone())
                                                .div(Nat::from(2u32)),
                                        );
                                        println!( "Too many logs received in range [{from}, {last_block_number}]. Will retry with range [{from}, {new_last_block_number}]");
                                        last_block_number = new_last_block_number;
                                        continue;
                                    }
                                }
                            }
                            _ => return None,
                        }
                    }
                };
            };

            for log_entry in logs {
                println!("Received event {log_entry:?}",);
                mutate_state(|s| s.record_log_to_process(&log_entry));
            }
            if read_state(State::has_logs_to_process) {
                println!("Found logs to process",);
                ic_cdk_timers::set_timer(Duration::from_secs(0), move || {
                    ic_cdk::spawn(process_logs())
                });
            }
            mutate_state(|s| s.last_scraped_block_number = last_block_number.clone());
            Some(last_block_number)
        }
        Ordering::Greater => {
            ic_cdk::trap(&format!(
              "BUG: last scraped block number ({:?}) is greater than the last queried block number ({:?})",
              from, to
          ));
        }
    }
}

pub async fn scrape_eth_logs() {
    let _guard = match TimerGuard::new(TaskType::ScrapeLogs) {
        Ok(guard) => guard,
        Err(_) => return,
    };

    let last_block_number = match update_last_observed_block_number().await {
        Some(block_number) => block_number,
        None => {
            println!(
                "[scrape_eth_logs]: skipping scraping ETH logs: no last observed block number"
            );
            return;
        }
    };

    let mut last_scraped_block_number = read_state(|s| s.last_scraped_block_number.clone());

    while last_scraped_block_number < last_block_number {
        let next_block_to_query = last_scraped_block_number.add(Nat::from(1u32));
        last_scraped_block_number =
            match scrape_eth_logs_range_inclusive(&next_block_to_query, &last_block_number).await {
                Some(last_scraped_block_number) => last_scraped_block_number,
                None => {
                    return;
                }
            };
    }
}

async fn update_last_observed_block_number() -> Option<Nat> {
    let rpc_providers = read_state(|s| s.rpc_services.clone());
    let block_tag = read_state(|s| s.block_tag.clone());

    let cycles = 10_000_000_000;
    let (result,) = EVM_RPC
        .eth_get_block_by_number(rpc_providers, None, block_tag, cycles)
        .await
        .expect("Call failed");

    match result {
        MultiGetBlockByNumberResult::Consistent(r) => match r {
            GetBlockByNumberResult::Ok(latest_block) => {
                let block_number = Some(latest_block.number);
                mutate_state(|s| s.last_observed_block_number = block_number.clone());
                block_number
            }
            GetBlockByNumberResult::Err(err) => {
                println!("Failed to get the latest finalized block number: {err:?}");
                read_state(|s| s.last_observed_block_number.clone())
            }
        },
        MultiGetBlockByNumberResult::Inconsistent(_) => {
            panic!("RPC providers gave inconsistent results")
        }
    }
}

impl HttpOutcallError {
    pub fn is_response_too_large(&self) -> bool {
        match self {
            Self::IcError { code, message } => is_response_too_large(code, message),
            _ => false,
        }
    }
}

pub fn is_response_too_large(code: &RejectionCode, message: &str) -> bool {
    code == &RejectionCode::SysFatal && message.contains("size limit")
}
