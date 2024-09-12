use ethers_core::{abi::Token, types::U256};
use evm_rpc_canister_types::{SendRawTransactionStatus, EVM_RPC};
use ic_evm_utils::eth_send_raw_transaction::{contract_interaction, ContractDetails};

use crate::state::{mutate_state, read_state, State};

pub async fn submit_result(job_id: U256) {
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
            if transaction_hash
                .expect("transaction hash should be present")
                .contains("-32010")
            {
                ic_cdk::println!("Transaction already known, assuming success");
            }
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
