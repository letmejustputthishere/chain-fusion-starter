use ethers_core::{
    types::{Address, Eip1559TransactionRequest, U256},
    utils::keccak256,
};
use evm_rpc_canister_types::SendRawTransactionStatus;
use ic_evm_utils::{
    eth_send_raw_transaction::send_raw_transaction,
    fees::{estimate_transaction_fees, FeeEstimates},
};
use ic_evm_utils::{eth_send_raw_transaction::IntoChainId, evm_signer::sign_eip1559_transaction};

use crate::state::{mutate_state, read_state, State};
use ethers_core::abi::AbiEncode;
use std::str::FromStr;

pub async fn submit_result(result: String, job_id: U256) {
    // get necessary global state
    let contract_address = &read_state(State::get_logs_addresses)[0];
    let rpc_services = read_state(State::rpc_services);
    let nonce = read_state(State::nonce);
    let key_id = read_state(State::key_id);

    //TODO: Should probably be hardcoded. Recomputing the hash every time is unnecessary
    let function_signature = "callback(string,uint256)";

    // as required,  provide the first 4 bytes of the hash of the invoked method signature and encoded parameters
    let selector = &keccak256(function_signature.as_bytes())[0..4];
    let args = (result, job_id).encode();
    let mut data = Vec::from(selector);
    data.extend(args);

    // set the gas
    let gas = Some(U256::from(5000000));
    // estimate the fees, this makes a call to the EVM RPC provider under the hood
    let FeeEstimates {
        max_fee_per_gas,
        max_priority_fee_per_gas,
    } = estimate_transaction_fees(9, rpc_services.clone()).await;

    // assemble the transaction
    let tx = Eip1559TransactionRequest {
        to: Some(
            Address::from_str(contract_address)
                .expect("should be a valid address")
                .into(),
        ),
        gas,
        data: Some(data.into()),
        nonce: Some(nonce),
        max_priority_fee_per_gas: Some(max_priority_fee_per_gas),
        max_fee_per_gas: Some(max_fee_per_gas),
        chain_id: Some(rpc_services.chain_id()),
        from: Default::default(),
        value: Default::default(),
        access_list: Default::default(),
    };

    // sign the transaction using chain key signatures
    let tx = sign_eip1559_transaction(tx, key_id, vec![]).await;

    // send the transaction via the EVM RPC canister
    let status = send_raw_transaction(tx, rpc_services).await;

    // if the transaction
    match status {
        SendRawTransactionStatus::Ok(transaction_hash) => {
            ic_cdk::println!("Success {transaction_hash:?}");
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
