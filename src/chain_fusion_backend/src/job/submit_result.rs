use ethers_core::{types::U256, utils::keccak256};

use crate::{
    evm_rpc::SendRawTransactionStatus,
    evm_signer,
    fees,
    state::{mutate_state, read_state},
    eth_send_raw_transaction::{create_sign_request, send_raw_transaction},
};
use ethers_core::abi::AbiEncode;


// todo: properly name this function 
pub async fn submit_result(job_id: U256) {
    let function_signature = "executeJob(uint256)";

    let selector = &keccak256(function_signature.as_bytes())[0..4];
    let args = (job_id).encode();
    let mut data = Vec::from(selector);
    data.extend(args);

    let gas_limit = U256::from(5000000);
    let fee_estimates = fees::estimate_transaction_fees(9).await;

    let contract_address = read_state(|s| s.get_logs_address[0].clone());

    let request = create_sign_request(
        U256::from(0),
        Some(contract_address),
        None,
        gas_limit,
        Some(data),
        fee_estimates,
    )
    .await;

    let tx = evm_signer::sign_transaction(request).await;

    let status = send_raw_transaction(tx.clone()).await;

    println!("Transaction sent: {:?}", tx);

    match status {
        SendRawTransactionStatus::Ok(transaction_hash) => {
            println!("Success {transaction_hash:?}");
            mutate_state(|s| {
                s.nonce += U256::from(1);
            });
        }
        SendRawTransactionStatus::NonceTooLow => {
            println!("Nonce too low");
        }
        SendRawTransactionStatus::NonceTooHigh => {
            println!("Nonce too high");
        }
        SendRawTransactionStatus::InsufficientFunds => {
            println!("Insufficient funds");
        }
    }
}
