//! This module contains functions for interacting with Ethereum contracts using JSON-RPC requests.
use ethers_core::abi::Token;
use ethers_core::types::U256;
use ethers_core::utils::hex;
use hex::FromHexError;
use serde::{Deserialize, Serialize};

use evm_rpc_canister_types::{EvmRpcCanister, RequestResult, RpcService};

use crate::eth_send_raw_transaction::{get_data, get_function, ContractDetails};
use crate::request::{request, JsonRpcResult};

/// Represents the parameters for an Ethereum call.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EthCallParams {
    pub to: String,
    pub data: String,
}

/// Represents a JSON-RPC request for an Ethereum call.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EthCallJsonRpcRequest {
    pub id: u64,
    pub jsonrpc: String,
    pub method: String,
    pub params: (EthCallParams, String),
}

/// Executes an Ethereum call.
///
/// # Arguments
///
/// * `contract_details` - The details of the contract to call.
/// * `block_number` - The block number to execute the call on.
/// * `rpc_service` - The RPC service to use for the call.
/// * `max_response_bytes` - The maximum number of response bytes to accept.
/// * `evm_rpc` - The EVM RPC canister.
///
/// # Returns
///
/// The decoded output of the call as a vector of tokens.
pub async fn eth_call(
    contract_details: ContractDetails<'_>,
    block_number: &str,
    rpc_service: RpcService,
    max_response_bytes: u64,
    evm_rpc: EvmRpcCanister,
) -> Vec<Token> {
    let function = get_function(&contract_details);
    let data = get_data(function, &contract_details);
    let json_rpc_payload = serde_json::to_string(&EthCallJsonRpcRequest {
        id: 1,
        jsonrpc: "2.0".to_string(),
        method: "eth_call".to_string(),
        params: (
            EthCallParams {
                to: contract_details.contract_address.clone(),
                data: to_hex(&data),
            },
            block_number.to_string(),
        ),
    })
    .expect("Error while encoding JSON-RPC request");

    let res = request(rpc_service, json_rpc_payload, max_response_bytes, evm_rpc).await;

    match res {
        RequestResult::Ok(ok) => {
            let json: JsonRpcResult =
                serde_json::from_str(&ok).expect("JSON was not well-formatted");
            let result = from_hex(&json.result.expect("Unexpected JSON response")).unwrap();
            function
                .decode_output(&result)
                .expect("Error decoding output")
        }
        RequestResult::Err(err) => panic!("Response error: {err:?}"),
    }
}

/// Retrieves the balance of an ERC20 token for a given account.
///
/// # Arguments
///
/// * `contract_address` - The address of the ERC20 token contract.
/// * `account` - The account to retrieve the balance for.
/// * `rpc_service` - The RPC service to use for the call.
/// * `evm_rpc` - The EVM RPC canister.
///
/// # Returns
///
/// The balance of the ERC20 token for the given account.
pub async fn erc20_balance_of(
    contract_address: String,
    account: String,
    rpc_service: RpcService,
    evm_rpc: EvmRpcCanister,
) -> U256 {
    let max_response_bytes = 2048;
    // Define the ABI JSON as a string literal
    let abi_json = r#"
   [
       {
           "constant": true,
           "inputs": [
               {
                   "name": "_owner",
                   "type": "address"
               }
           ],
           "name": "balanceOf",
           "outputs": [
               {
                   "name": "balance",
                   "type": "uint256"
               }
           ],
           "type": "function"
       }
   ]
   "#;
    let abi =
        serde_json::from_str::<ethers_core::abi::Contract>(abi_json).expect("should serialise");

    let contract_details = ContractDetails {
        contract_address,
        abi: &abi,
        function_name: "balanceOf",
        args: &[Token::Address(
            account.parse().expect("address should be valid"),
        )],
    };

    let Token::Uint(balance) = eth_call(
        contract_details,
        "latest",
        rpc_service,
        max_response_bytes,
        evm_rpc,
    )
    .await
    .first()
    .unwrap()
    .clone() else {
        panic!("oops")
    };
    balance
}

/// Converts a byte slice to a hexadecimal string representation.
///
/// # Arguments
///
/// * `data` - The byte slice to convert.
///
/// # Returns
///
/// The hexadecimal string representation of the byte slice.
fn to_hex(data: &[u8]) -> String {
    format!("0x{}", hex::encode(data))
}

/// Converts a hexadecimal string representation to a byte slice.
///
/// # Arguments
///
/// * `data` - The hexadecimal string to convert.
///
/// # Returns
///
/// The byte slice representation of the hexadecimal string, or an error if the conversion fails.
fn from_hex(data: &str) -> Result<Vec<u8>, FromHexError> {
    hex::decode(&data[2..])
}
