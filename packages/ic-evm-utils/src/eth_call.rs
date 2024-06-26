use ethers_core::abi::{Contract, FunctionExt, Token};
use ethers_core::types::U256;
use ethers_core::utils::hex;
use hex::FromHexError;
use serde::{Deserialize, Serialize};

use evm_rpc_canister_types::{RequestResult, RpcService};

use crate::request::{request, JsonRpcResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EthCallParams {
    pub to: String,
    pub data: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EthCallJsonRpcRequest {
    pub id: u64,
    pub jsonrpc: String,
    pub method: String,
    pub params: (EthCallParams, String),
}

#[allow(dead_code)]
async fn eth_call(
    contract_address: String,
    abi: &Contract,
    function_name: &str,
    args: &[Token],
    block_number: &str,
    rpc_service: RpcService,
    max_response_bytes: u64,
) -> Vec<Token> {
    let function = match abi.functions_by_name(function_name).map(|v| &v[..]) {
        Ok([f]) => f,
        Ok(fs) => panic!(
            "Found {} function overloads. Please pass one of the following: {}",
            fs.len(),
            fs.iter()
                .map(|f| format!("{:?}", f.abi_signature()))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Err(_) => abi
            .functions()
            .find(|f| function_name == f.abi_signature())
            .expect("Function not found"),
    };
    let data = function
        .encode_input(args)
        .expect("Error while encoding input args");
    let json_rpc_payload = serde_json::to_string(&EthCallJsonRpcRequest {
        id: 1,
        jsonrpc: "2.0".to_string(),
        method: "eth_call".to_string(),
        params: (
            EthCallParams {
                to: contract_address,
                data: to_hex(&data),
            },
            block_number.to_string(),
        ),
    })
    .expect("Error while encoding JSON-RPC request");

    let res = request(rpc_service, json_rpc_payload, max_response_bytes).await;

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

#[allow(dead_code)]
pub async fn erc20_balance_of(
    contract_address: String,
    account: String,
    rpc_service: RpcService,
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

    let Token::Uint(balance) = eth_call(
        contract_address.to_string(),
        &abi,
        "balanceOf",
        &[Token::Address(
            account.parse().expect("address should be valid"),
        )],
        "latest",
        rpc_service,
        max_response_bytes,
    )
    .await
    .first()
    .unwrap()
    .clone() else {
        panic!("oops")
    };
    balance
}

fn to_hex(data: &[u8]) -> String {
    format!("0x{}", hex::encode(data))
}

fn from_hex(data: &str) -> Result<Vec<u8>, FromHexError> {
    hex::decode(&data[2..])
}
