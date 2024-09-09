use crate::logs::update_last_observed_block_number;
use ic_evm_utils::conversions::nat_to_u256;

use evm_rpc_canister_types::{
    BlockTag, GetTransactionCountArgs, GetTransactionCountResult, MultiGetTransactionCountResult,
    EVM_RPC,
};
use ic_cdk::println;

use crate::{
    guard::TimerGuard,
    state::{mutate_state, read_state, TaskType},
};

// after a (re-) install, load latest block number from the Ethereum network
#[allow(dead_code)]
pub async fn init_latest_block() {
    // prevent the scrape_eth_logs task from running until this one is done
    let _guard = match TimerGuard::new(TaskType::ScrapeLogs) {
        Ok(guard) => guard,
        Err(_) => return,
    };

    // load latest block's number
    let last_block_number = match update_last_observed_block_number().await {
        Some(block_number) => block_number,
        None => {
            println!("[init_latest_block]: unable to load the latest block number");
            return;
        }
    };
    // store the latest block number as the last scraped block number
    println!("[init_latest_block]: loaded the latest block number: {last_block_number}");
    mutate_state(|s| s.last_scraped_block_number.clone_from(&last_block_number));
}

// after a (re-) install, load the account nonce from the Ethereum network
pub async fn init_nonce() {
    // prevent the scrape_eth_logs task from running until this one is done
    let _guard = match TimerGuard::new(TaskType::ScrapeLogs) {
        Ok(guard) => guard,
        Err(_) => return,
    };

    let evm_address: String =
        read_state(|s| s.evm_address.clone()).expect("evm address should be initialized");

    let get_transaction_count_args: GetTransactionCountArgs = GetTransactionCountArgs {
        address: evm_address.clone(),
        block: BlockTag::Latest,
    };

    let cycles = 10_000_000_000;

    // load the account nonce
    let (result,) = EVM_RPC
        .eth_get_transaction_count(
            read_state(|s| s.rpc_services.clone()),
            None,
            get_transaction_count_args.clone(),
            cycles,
        )
        .await
        .expect("Call failed (init_nonce)");

    match result {
        MultiGetTransactionCountResult::Consistent(r) => match r {
            GetTransactionCountResult::Ok(nonce) => {
                println!("[init_nonce]: loaded the account nonce: {nonce}");
                let nonce_u256 = nat_to_u256(&nonce);
                mutate_state(|s| s.nonce.clone_from(&nonce_u256));
            }
            GetTransactionCountResult::Err(err) => {
                println!("[init_nonce]: failed to get the account nonce: {err:?}");
                return;
            }
        },
        MultiGetTransactionCountResult::Inconsistent(_) => {
            println!("[init_nonce]: RPC providers gave inconsistent results");
            return;
        }
    }
}
