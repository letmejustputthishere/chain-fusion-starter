use candid::Nat;
use ethers_core::types::{U256, U64};
use ic_cdk::println;
use serde_bytes::ByteBuf;
use std::ops::Add;

use crate::{
    evm_rpc::{
        BlockTag, MultiSendRawTransactionResult, RpcServices, SendRawTransactionResult,
        SendRawTransactionStatus, EVM_RPC,
    },
    evm_signer::{self, SignRequest},
    fees,
    state::{mutate_state, read_state},
};

fn nat_to_u256(n: &Nat) -> U256 {
    let be_bytes = n.0.to_bytes_be();
    U256::from_big_endian(&be_bytes)
}

async fn estimate_transaction_fees() -> (U256, U256) {
    // we are setting the `max_priority_fee_per_gas` based on this article:
    // https://docs.alchemy.com/docs/maxpriorityfeepergas-vs-maxfeepergas
    // following this logic, the base fee will be derived from the block history automatically
    // and we only specify the maximum priority fee per gas (tip).
    // the tip is derived from the fee history of the last 9 blocks, more specifically
    // from the 95th percentile of the tip.
    let fee_history = fees::fee_history(
        Nat::from(9u32),
        BlockTag::Latest,
        Some(ByteBuf::from(vec![95])),
    )
    .await;

    // baseFeePerGas median over the past 9 blocks
    let mut base_fee_per_gas = fee_history.baseFeePerGas;
    // sort the base fees in ascending order
    base_fee_per_gas.sort_unstable();
    // get the median by accessing the element in the middle
    let base_fee = base_fee_per_gas
        .get(4)
        .expect("the base_fee_per_gas should have 9 elements")
        .clone();

    // obtain the 95th percentile of the tips for the past 9 blocks
    let mut percentile_95: Vec<Nat> = fee_history
        .reward
        .into_iter()
        .flat_map(|x| x.into_iter())
        .collect();
    // sort the tips in ascending order
    percentile_95.sort_unstable();
    // get the median by accessing the element in the middle
    let max_priority_fee_per_gas = percentile_95
        .get(4)
        .expect("the 95th percentile should have 9 elements")
        .clone();

    let max_fee_per_gas = max_priority_fee_per_gas.clone().add(base_fee);

    (
        nat_to_u256(&max_fee_per_gas),
        nat_to_u256(&max_priority_fee_per_gas),
    )
}

pub async fn transfer_eth_from_canister(value: u128, to: String) {
    let (max_fee_per_gas, max_priority_fee_per_gas) = estimate_transaction_fees().await;
    let nonce = read_state(|s| s.nonce);
    let rpc_providers = read_state(|s| s.rpc_services.clone());

    let req = SignRequest {
        chain_id: Some(rpc_providers.chain_id()),
        to: Some(to),
        from: None,
        gas: Some(U256::from(21000)),
        max_fee_per_gas: Some(max_fee_per_gas),
        max_priority_fee_per_gas: Some(max_priority_fee_per_gas),
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
