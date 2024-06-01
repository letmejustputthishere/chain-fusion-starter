use std::collections::BTreeMap;
use std::time::Duration;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use ethers_core::abi::ethereum_types::{Address};
use ic_cdk::{update};

const CANISTER_ETH_ADDRESS: String = '';

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    static MAP: RefCell<StableBTreeMap<u128, ExecutionTimeRecord, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
}

struct ExecutionTimeRecord {
    pub frequency_in_milliseconds: u64;
    pub account_to_send_tokens_to: Address;
    pub account_to_send_tokens_from: Address;
    pub token_amount_to_send: u64;
    pub token_contract_address: Address;
    pub timer_id: String;
}

#[ic_cdk_macros::update]
fn create_reccuring_transaction(
    token_contract_address: String,
    token_amount_to_send: u64,
    frequency_in_milliseconds: u64,
    account_to_send_tokens_from: String,
    account_to_send_tokens_to: String
) -> Option<u128> {
    const exec_duration: Duration = Duration::from_secs(frequency_in_milliseconds);
    const exec_timer_id = ic_cdk_timers::set_timer_interval(exec_duration, execute);
    let new_transaction = ExecutionTimeRecord {
        frequency_in_milliseconds: frequency_in_milliseconds,
        token_amount_to_send: token_amount_to_send,
        token_contract_address: Address::from_str(&token_contract_address)
                .expect("failed to parse the source address")
                .into(),
        account_to_send_tokens_from: Address::from_str(&account_to_send_tokens_from)
            .expect("failed to parse the source address")
            .into(),
        account_to_send_tokens_to:  Address::from_str(&account_to_send_tokens_to)
            .expect("failed to parse the source address")
            .into(),
        timer_id: exec_timer_id
    };

    fn execute() {
        print!("payment to account {0} executed with amount {1}", account_to_send_tokens_to, token_amount_to_send);
    }

    MAP.with(|p| p.borrow_mut().insert(123, new_transaction))
}

#[ic_cdk_macros::update]
fn remove_recurring_transaction(uuid: u128) -> Option<u128> {
    MAP.with(|p| p.borrow_mut().remove(uuid))
}
