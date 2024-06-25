use crate::evm_signer::{self, IntoChainId};
use alloy::{
    consensus::TxEip1559,
    primitives::{TxKind, U256},
};
use evm_rpc_canister_types::{
    MultiSendRawTransactionResult, RpcServices, SendRawTransactionResult, SendRawTransactionStatus,
    EVM_RPC,
};
use ic_cdk::api::management_canister::ecdsa::EcdsaKeyId;

use crate::fees::estimate_transaction_fees;

pub async fn transfer_eth(
    value: U256,
    to: TxKind,
    gas_limit: Option<u128>,
    rpc_services: RpcServices,
    key_id: EcdsaKeyId,
    derivation_path: Vec<Vec<u8>>,
    nonce: u64,
) -> SendRawTransactionStatus {
    // use the user provided gas_limit or fallback to default 21000
    let gas_limit = gas_limit.unwrap_or(21000);
    // estimate the transaction fees by calling eth_feeHistory
    let fee_estimates = estimate_transaction_fees(9, rpc_services.clone()).await;
    // assemble the EIP 1559 transaction to be signed with t-ECDSA
    let tx = TxEip1559 {
        chain_id: rpc_services.chain_id(),
        gas_limit,
        to,
        max_fee_per_gas: fee_estimates.max_fee_per_gas,
        max_priority_fee_per_gas: fee_estimates.max_priority_fee_per_gas,
        value,
        nonce,
        access_list: Default::default(),
        input: Default::default(),
    };

    let tx = evm_signer::sign_eip1559_transaction(tx, key_id, derivation_path).await;

    send_raw_transaction(tx, rpc_services).await

    // match status {
    //     SendRawTransactionStatus::Ok(transaction_hash) => {
    //         println!("Success {transaction_hash:?}");
    //         mutate_state(|s| {
    //             s.nonce += U256::from(1);
    //         });
    //     }
    //     SendRawTransactionStatus::NonceTooLow => {
    //         println!("Nonce too low");
    //     }
    //     SendRawTransactionStatus::NonceTooHigh => {
    //         println!("Nonce too high");
    //     }
    //     SendRawTransactionStatus::InsufficientFunds => {
    //         println!("Insufficient funds");
    //     }
    // }
}

pub async fn send_raw_transaction(
    tx: String,
    rpc_services: RpcServices,
) -> SendRawTransactionStatus {
    let cycles = 10_000_000_000;

    match EVM_RPC
        .eth_send_raw_transaction(rpc_services, None, tx, cycles)
        .await
    {
        Ok((res,)) => match res {
            MultiSendRawTransactionResult::Consistent(status) => match status {
                SendRawTransactionResult::Ok(status) => status,
                SendRawTransactionResult::Err(e) => {
                    ic_cdk::trap(format!("Error: {:?}", e).as_str());
                }
            },
            MultiSendRawTransactionResult::Inconsistent(_) => {
                ic_cdk::trap("Status is inconsistent");
            }
        },
        Err(e) => ic_cdk::trap(format!("Error: {:?}", e).as_str()),
    }
}
