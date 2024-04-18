use candid::Nat;
use ethers_core::types::U256;
use ic_cdk::println;
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

pub async fn transfer_eth_from_canister(value: u128, to: String) {
    let fee_history = fees::fee_history(Nat::from(10u32), BlockTag::Latest, None).await;
    let base_fee = fee_history.baseFeePerGas.last().unwrap().clone();

    let max_priority_fee_per_gas = 100u32;
    let max_fee_per_gas = base_fee.add(max_priority_fee_per_gas).0;

    let nonce = read_state(|s| s.nonce);
    let rpc_providers = read_state(|s| s.rpc_services.clone());

    let req = SignRequest {
        chain_id: rpc_providers.chain_id(),
        to,
        gas: U256::from(50000),
        max_fee_per_gas: U256::from(u128::try_from(max_fee_per_gas).unwrap()),
        max_priority_fee_per_gas: U256::from(max_priority_fee_per_gas),
        data: None,
        value: U256::from(value),
        nonce: U256::from(nonce),
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
    pub fn chain_id(&self) -> u64 {
        match self {
            RpcServices::EthSepolia(_) => 11155111,
            RpcServices::Custom {
                chainId,
                services: _,
            } => *chainId,
            RpcServices::EthMainnet(_) => 1,
        }
    }
}
