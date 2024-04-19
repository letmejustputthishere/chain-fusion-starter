use candid::Nat;
use ethers_core::types::{U256, U64};
use ic_cdk::println;
use serde_bytes::ByteBuf;

use crate::{
    evm_rpc::{
        BlockTag, MultiSendRawTransactionResult, RpcServices, SendRawTransactionResult,
        SendRawTransactionStatus, EVM_RPC,
    },
    evm_signer::{self, SignRequest},
    fees,
    state::{mutate_state, read_state},
};

pub async fn transfer_eth_from_canister(value: u128, to: String) {
    // we are setting the `max_priority_fee_per_gas` based on this article:
    // https://docs.alchemy.com/docs/maxpriorityfeepergas-vs-maxfeepergas
    // following this logic, the base fee will be derived from the block history automatically
    // and we only specify the maximum priority fee per gas (tip).
    // the tip is derived from the fee history of the last 9 blocks, more specifically
    // from the 95th percentile of the tip.
    let mut fee_history = fees::fee_history(
        Nat::from(9u32),
        BlockTag::Latest,
        Some(ByteBuf::from(vec![95])),
    )
    .await;

    // obtain the 95th percentile of the tips for the past 9 blocks
    let percentile_95 = fee_history
        .reward
        .first_mut()
        .expect("the rewards should be present as we supply the 95th percentile to `fee_history`");
    // sort the tips in ascending order
    percentile_95.sort_unstable();
    // get the median by accessing the element in the middle
    let max_priority_fee_per_gas = percentile_95
        .get(4)
        .expect("the 95th percentile should have 9 elements");

    let nonce = read_state(|s| s.nonce);
    let rpc_providers = read_state(|s| s.rpc_services.clone());

    let req = SignRequest {
        chain_id: Some(rpc_providers.chain_id()),
        to: Some(to),
        from: None,
        gas: Some(U256::from(21000)),
        max_fee_per_gas: None,
        max_priority_fee_per_gas: Some(U256::from_big_endian(
            &max_priority_fee_per_gas.0.to_bytes_be(),
        )),
        data: None,
        value: Some(U256::from(value)),
        nonce: Some(U256::from(nonce)),
    };

    let tx = evm_signer::sign_transaction(req).await;

    let status = send_raw_transaction(tx.clone()).await;

    println!("Transaction sent: {:?}", tx);

    match status {
        SendRawTransactionStatus::Ok(transaction_hash) => {
            println!("Success {transaction_hash:?}");
            mutate_state(|s| {
                s.nonce += 1;
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

pub async fn send_raw_transaction(tx: String) -> SendRawTransactionStatus {
    let rpc_providers = read_state(|s| s.rpc_services.clone());
    let cycles = 10_000_000_000;

    match EVM_RPC
        .eth_send_raw_transaction(rpc_providers, None, tx, cycles)
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

impl RpcServices {
    pub fn chain_id(&self) -> U64 {
        match self {
            RpcServices::EthSepolia(_) => U64::from(11155111),
            RpcServices::Custom {
                chainId,
                services: _,
            } => U64::from(*chainId),
            RpcServices::EthMainnet(_) => U64::from(1),
        }
    }
}
