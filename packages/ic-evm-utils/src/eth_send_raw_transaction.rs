use ethers_core::abi::{Address, Contract, Function, FunctionExt, Token};
use ethers_core::types::{Eip1559TransactionRequest, NameOrAddress, U256, U64};
use evm_rpc_canister_types::{
    EvmRpcCanister, MultiSendRawTransactionResult, RpcServices, SendRawTransactionResult,
    SendRawTransactionStatus,
};
use ic_cdk::api::management_canister::ecdsa::EcdsaKeyId;
use std::str::FromStr;

use crate::{
    evm_signer::sign_eip1559_transaction,
    fees::{estimate_transaction_fees, FeeEstimates},
};

pub struct TransferArgs {
    pub value: U256,
    pub to: Option<NameOrAddress>,
    pub gas: Option<U256>,
}

/// Make sure to increase the nonce if the transfer was successfull
pub async fn transfer_eth(
    transfer_args: TransferArgs,
    rpc_services: RpcServices,
    key_id: EcdsaKeyId,
    derivation_path: Vec<Vec<u8>>,
    nonce: U256,
    evm_rpc: EvmRpcCanister,
) -> SendRawTransactionStatus {
    // use the user provided gas_limit or fallback to default 210000
    let gas = transfer_args.gas.unwrap_or(U256::from(21000));
    // estimate the transaction fees by calling eth_feeHistory
    let FeeEstimates {
        max_fee_per_gas,
        max_priority_fee_per_gas,
    } = estimate_transaction_fees(9, rpc_services.clone(), evm_rpc.clone()).await;
    // assemble the EIP 1559 transaction to be signed with t-ECDSA
    let tx = Eip1559TransactionRequest {
        from: None,
        to: transfer_args.to,
        value: Some(transfer_args.value),
        max_fee_per_gas: Some(max_fee_per_gas),
        max_priority_fee_per_gas: Some(max_priority_fee_per_gas),
        gas: Some(gas),
        nonce: Some(nonce),
        chain_id: Some(rpc_services.chain_id()),
        data: Default::default(),
        access_list: Default::default(),
    };

    let tx = sign_eip1559_transaction(tx, key_id, derivation_path).await;

    send_raw_transaction(tx.clone(), rpc_services, evm_rpc).await
}

pub struct ContractDetails<'a> {
    pub contract_address: String,
    pub abi: &'a Contract,
    pub function_name: &'a str,
    pub args: &'a [Token],
}

pub fn get_function<'a>(contract_details: &'a ContractDetails<'a>) -> &'a Function {
    match contract_details
        .abi
        .functions_by_name(contract_details.function_name)
        .map(|v| &v[..])
    {
        Ok([f]) => f,
        Ok(fs) => panic!(
            "Found {} function overloads. Please pass one of the following: {}",
            fs.len(),
            fs.iter()
                .map(|f| format!("{:?}", f.abi_signature()))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Err(_) => contract_details
            .abi
            .functions()
            .find(|f| contract_details.function_name == f.abi_signature())
            .expect("Function not found"),
    }
}

pub fn get_data<'a>(function: &Function, contract_details: &'a ContractDetails<'a>) -> Vec<u8> {
    function
        .encode_input(contract_details.args)
        .expect("Error while encoding input args")
}

pub async fn contract_interaction(
    contract_details: ContractDetails<'_>,
    gas: Option<U256>,
    rpc_services: RpcServices,
    nonce: U256,
    key_id: EcdsaKeyId,
    derivation_path: Vec<Vec<u8>>,
    evm_rpc: EvmRpcCanister,
) -> SendRawTransactionStatus {
    let function = get_function(&contract_details);
    let data = get_data(function, &contract_details);

    let FeeEstimates {
        max_fee_per_gas,
        max_priority_fee_per_gas,
    } = estimate_transaction_fees(9, rpc_services.clone(), evm_rpc.clone()).await;

    // assemble the transaction
    let tx = Eip1559TransactionRequest {
        to: Some(
            Address::from_str(&contract_details.contract_address)
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
    let tx = sign_eip1559_transaction(tx, key_id, derivation_path).await;

    // send the transaction via the EVM RPC canister
    send_raw_transaction(tx, rpc_services, evm_rpc).await
}

pub async fn send_raw_transaction(
    tx: String,
    rpc_services: RpcServices,
    evm_rpc: EvmRpcCanister,
) -> SendRawTransactionStatus {
    let cycles = 10_000_000_000;

    match evm_rpc
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
