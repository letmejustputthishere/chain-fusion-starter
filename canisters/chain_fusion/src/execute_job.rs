use ethers_core::{abi::Token, types::U256};
use evm_rpc_canister_types::{SendRawTransactionStatus, EVM_RPC};
use ic_cdk::api;
use ic_evm_utils::eth_send_raw_transaction::{contract_interaction, ContractDetails};

use crate::guard::TimerGuard;
use crate::state::{mutate_state, read_state, State, TaskType};

pub async fn execute_jobs() {
    // the timer guard prevent simultaneous execution of multiple instances of the same task
    let _guard = match TimerGuard::new(TaskType::ExecuteJobs) {
        Ok(guard) => guard,
        Err(_) => return,
    };

    // check if all logs have been processed
    let logs_to_process = read_state(|s| (s.logs_to_process.clone()));
    if !logs_to_process.is_empty() {
        ic_cdk::println!("Skipping execution of jobs because there are logs to process");
        return;
    }

    // check if there are any jobs to execute, and execute them if they are ready
    let current_timestamp = api::time() / 1_000_000_000; // converted to seconds
    let earliest_job = read_state(|s| s.get_earliest_job());
    if let Some((job_id, job_execution_time)) = earliest_job {
        if job_execution_time <= current_timestamp {
            ic_cdk::println!("Executing job with ID: {:?}", job_id);
            execute_job(job_id).await;
            mutate_state(|s| s.remove_job(&job_id)); // remove the job from the queue
            ic_cdk::println!("Executed job with ID: {:?}", job_id);
        }
    } else {
        ic_cdk::println!("No jobs to execute");
    }
}

pub async fn execute_job(job_id: U256) {
    println!("Executing job {job_id} now.");

    // get necessary global state
    let contract_address = &read_state(State::get_logs_addresses)[0];
    let rpc_services = read_state(State::rpc_services);
    let nonce = read_state(State::nonce);
    let key_id = read_state(State::key_id);

    let abi_json = r#"
   [
        {
            "type": "function",
            "name": "executeJob",
            "inputs": [
                {
                    "name": "_job_id",
                    "type": "uint256",
                    "internalType": "uint256"
                }
            ],
            "outputs": [],
            "stateMutability": "nonpayable"
        }
   ]
   "#;

    let abi =
        serde_json::from_str::<ethers_core::abi::Contract>(abi_json).expect("should serialise");

    let contract_details = ContractDetails {
        contract_address: contract_address.clone(),
        abi: &abi,
        function_name: "executeJob",
        args: &[Token::Uint(job_id)],
    };

    println!("Executing job_id: {job_id} on contract {contract_address}");

    // set the gas
    let gas = Some(U256::from(1000000));

    // interact with the contract, this calls `eth_sendRawTransaction` under the hood
    let status = contract_interaction(
        contract_details,
        gas,
        rpc_services,
        nonce,
        key_id,
        vec![],
        EVM_RPC,
    )
    .await;

    match status {
        SendRawTransactionStatus::Ok(transaction_hash) => {
            ic_cdk::println!("Success {transaction_hash:?}");
            ic_cdk::println!("Used nonce {nonce}");
            mutate_state(|s| {
                s.nonce += U256::from(1);
            });
        }

        SendRawTransactionStatus::AlreadyKnown => {
            ic_cdk::println!("Transaction already known, assuming success");
            ic_cdk::println!("Used nonce {nonce}");
            mutate_state(|s| {
                s.nonce += U256::from(1);
            });
        }

        SendRawTransactionStatus::NonceTooLow => {
            ic_cdk::println!("Nonce too low");
        }
        SendRawTransactionStatus::NonceTooHigh => {
            ic_cdk::println!("Nonce too high");
        }
        SendRawTransactionStatus::InsufficientFunds => {
            ic_cdk::println!("Insufficient funds");
        }
    }
}
