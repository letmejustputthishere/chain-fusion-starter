use crate::evm_rpc::{BlockTag, LogEntry, RpcServices};

use candid::Nat;
use ethers_core::types::U256;
use ic_cdk::api::management_canister::ecdsa::EcdsaKeyId;
use std::collections::{BTreeMap, BTreeSet, HashSet};

use std::cell::RefCell;

thread_local! {
    pub static STATE: RefCell<Option<State>> = RefCell::default();
}

#[derive(Debug, Clone)]
pub struct State {
    pub rpc_services: RpcServices,
    pub get_logs_address: Vec<String>,
    pub get_logs_topics: Option<Vec<Vec<String>>>,
    pub last_scraped_block_number: Nat,
    pub last_observed_block_number: Option<Nat>,
    pub logs_to_process: BTreeMap<LogSource, LogEntry>,
    pub processed_logs: BTreeMap<LogSource, LogEntry>,
    pub skipped_blocks: BTreeSet<Nat>,
    pub active_tasks: HashSet<TaskType>,
    pub ecdsa_pub_key: Option<Vec<u8>>,
    pub ecdsa_key_id: EcdsaKeyId,
    pub evm_address: Option<String>,
    pub nonce: U256,
    pub block_tag: BlockTag,
}

#[derive(Debug, Eq, PartialEq)]
pub enum InvalidStateError {
    InvalidEthereumContractAddress(String),
    InvalidTopic(String),
}

impl State {
    pub fn record_log_to_process(&mut self, log_entry: &LogEntry) {
        let event_source = log_entry.source();
        assert!(
            !self.logs_to_process.contains_key(&event_source),
            "there must be no two different events with the same source"
        );
        assert!(!self.processed_logs.contains_key(&event_source));

        self.logs_to_process.insert(event_source, log_entry.clone());
    }

    pub fn record_processed_log(&mut self, source: LogSource) {
        let log_entry = match self.logs_to_process.remove(&source) {
            Some(event) => event,
            None => panic!("attempted to run job for an unknown event {source:?}"),
        };

        assert_eq!(
            self.processed_logs.insert(source.clone(), log_entry),
            None,
            "attempted to run job twice for the same event {source:?}"
        );
    }

    pub fn record_skipped_block(&mut self, block_number: Nat) {
        assert!(
            self.skipped_blocks.insert(block_number.clone()),
            "BUG: block {} was already skipped",
            block_number
        );
    }

    pub fn has_logs_to_process(&self) -> bool {
        !self.logs_to_process.is_empty()
    }
}

impl LogEntry {
    pub fn source(&self) -> LogSource {
        LogSource {
            transaction_hash: self
                .transactionHash
                .clone()
                .expect("for finalized blocks logs are not pending"),
            log_index: self
                .logIndex
                .clone()
                .expect("for finalized blocks logs are not pending"),
        }
    }
}

/// A unique identifier of the event source: the source transaction hash and the log
/// entry index.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogSource {
    pub transaction_hash: String,
    pub log_index: Nat,
}

pub fn read_state<R>(f: impl FnOnce(&State) -> R) -> R {
    STATE.with(|s| f(s.borrow().as_ref().expect("BUG: state is not initialized")))
}

/// Mutates (part of) the current state using `f`.
///
/// Panics if there is no state.
pub fn mutate_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut State) -> R,
{
    STATE.with(|s| {
        f(s.borrow_mut()
            .as_mut()
            .expect("BUG: state is not initialized"))
    })
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub enum TaskType {
    ProcessLogs,
    ScrapeLogs,
}
