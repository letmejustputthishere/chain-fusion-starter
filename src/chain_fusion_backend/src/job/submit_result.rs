use ethers_core::{types::U256, utils::keccak256};

use crate::{
    evm_rpc::SendRawTransactionStatus,
    evm_signer,
    state::{mutate_state, read_state},
    transactions::{create_sign_request, send_raw_transaction},
};
use ethers_core::abi::AbiEncode;

pub async fn submit_result(result: String, job_id: U256) {
    //TODO: Should probably be hardcoded. Recomputing the hash every time is unnecessary
    let function_signature = "callback(string,uint256)";

    // let mut data = keccak256(function_signature).as_ref()[0..4].to_vec();
    // data.extend(ethers_core::abi::AbiEncode::encode(result));
    let selector = &keccak256(function_signature.as_bytes())[0..4];
    let args = (result, job_id).encode();
    let mut data = Vec::from(selector);
    data.extend(args);

    let contract_address = read_state(|s| s.get_logs_address[0].clone());

    let request = create_sign_request(
        U256::from(0),
        Some(contract_address),
        None,
        // TODO: Set gas based on the contract
        Some(U256::from(50000)),
        Some(data),
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
