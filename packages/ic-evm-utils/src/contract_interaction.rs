use crate::{
    eth_send_raw_transaction::{send_raw_transaction, IntoChainId},
    evm_signer::sign_eip1559_transaction,
};
use ethers_core::{
    abi::{Address, Contract, FunctionExt, Token},
    types::{Eip1559TransactionRequest, U256},
};
use evm_rpc_canister_types::{RpcServices, SendRawTransactionStatus};
use ic_cdk::api::management_canister::ecdsa::EcdsaKeyId;
use std::str::FromStr;

use crate::fees::{estimate_transaction_fees, FeeEstimates};

pub struct ContractDetails<'a> {
    pub contract_address: String,
    pub abi: &'a Contract,
    pub function_name: &'a str,
    pub args: &'a [Token],
}

pub async fn contract_interaction(
    contract_details: ContractDetails<'_>,
    gas: Option<U256>,
    rpc_services: RpcServices,
    nonce: U256,
    key_id: EcdsaKeyId,
    derivation_path: Vec<Vec<u8>>,
) -> SendRawTransactionStatus {
    let function = match contract_details
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
    };
    let data = function
        .encode_input(contract_details.args)
        .expect("Error while encoding input args");

    let FeeEstimates {
        max_fee_per_gas,
        max_priority_fee_per_gas,
    } = estimate_transaction_fees(9, rpc_services.clone()).await;

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
    send_raw_transaction(tx, rpc_services).await
}
