use ethers_core::types::{Eip1559TransactionRequest, NameOrAddress, U256, U64};
use evm_rpc_canister_types::{
    MultiSendRawTransactionResult, RpcServices, SendRawTransactionResult, SendRawTransactionStatus,
    EVM_RPC,
};
use ic_cdk::api::management_canister::ecdsa::EcdsaKeyId;

use crate::{
    evm_signer::sign_eip1559_transaction,
    fees::{estimate_transaction_fees, FeeEstimates},
};

/// Make sure to increase the nonce if the transfer was successfull
#[allow(clippy::too_many_arguments)]
pub async fn transfer_eth(
    value: U256,
    to: Option<NameOrAddress>,
    gas: Option<U256>,
    rpc_services: RpcServices,
    key_id: EcdsaKeyId,
    derivation_path: Vec<Vec<u8>>,
    nonce: U256,
    mutate_nonce: impl FnOnce(), // Closure to mutate the nonce
) {
    // use the user provided gas_limit or fallback to default 210000
    let gas = gas.unwrap_or(U256::from(210000));
    // estimate the transaction fees by calling eth_feeHistory
    let FeeEstimates {
        max_fee_per_gas,
        max_priority_fee_per_gas,
    } = estimate_transaction_fees(9, rpc_services.clone()).await;
    // assemble the EIP 1559 transaction to be signed with t-ECDSA
    let tx = Eip1559TransactionRequest {
        from: None,
        to,
        value: Some(value),
        max_fee_per_gas: Some(max_fee_per_gas),
        max_priority_fee_per_gas: Some(max_priority_fee_per_gas),
        gas: Some(gas),
        nonce: Some(nonce),
        chain_id: Some(rpc_services.chain_id()),
        data: Default::default(),
        access_list: Default::default(),
    };

    let tx = sign_eip1559_transaction(tx, key_id, derivation_path).await;

    let status = send_raw_transaction(tx.clone(), rpc_services).await;

    match status {
        SendRawTransactionStatus::Ok(transaction_hash) => {
            ic_cdk::println!("Success {transaction_hash:?}");
            mutate_nonce();
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

pub trait IntoChainId {
    fn chain_id(&self) -> U64;
}

impl IntoChainId for RpcServices {
    fn chain_id(&self) -> U64 {
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
