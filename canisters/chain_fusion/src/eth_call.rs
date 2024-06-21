use ethers_core::abi::{Contract, FunctionExt, Token};
use ethers_core::types::U256;
use hex::FromHexError;
use serde::{Deserialize, Serialize};

use crate::evm_rpc::{RequestResult, EVM_RPC};
use crate::state::read_state;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EthCallParams {
    pub to: String,
    pub data: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub id: u64,
    pub jsonrpc: String,
    pub method: String,
    pub params: (EthCallParams, String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonRpcResult {
    result: Option<String>,
    error: Option<JsonRpcError>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    code: isize,
    message: String,
}

async fn eth_call(
    contract_address: String,
    abi: &Contract,
    function_name: &str,
    args: &[Token],
    block_number: &str,
) -> Vec<Token> {
    let f = match abi.functions_by_name(function_name).map(|v| &v[..]) {
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
    let data = f
        .encode_input(args)
        .expect("Error while encoding input args");
    let json_rpc_payload = serde_json::to_string(&JsonRpcRequest {
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

    let rpc_provider = read_state(|s| s.rpc_service.clone());
    let max_response_bytes = 2048;
    let cycles = 10_000_000_000;

    let res = match EVM_RPC
        .request(rpc_provider, json_rpc_payload, max_response_bytes, cycles)
        .await
    {
        Ok((res,)) => res,
        Err(e) => ic_cdk::trap(format!("Error: {:?}", e).as_str()),
    };

    match res {
        RequestResult::Ok(ok) => {
            let json: JsonRpcResult =
                serde_json::from_str(&ok).expect("JSON was not well-formatted");
            let result = from_hex(&json.result.expect("Unexpected JSON response")).unwrap();
            f.decode_output(&result).expect("Error decoding output")
        }
        RequestResult::Err(err) => panic!("Response error: {err:?}"),
    }
}

fn to_hex(data: &[u8]) -> String {
    format!("0x{}", hex::encode(data))
}

fn from_hex(data: &str) -> Result<Vec<u8>, FromHexError> {
    hex::decode(&data[2..])
}

pub async fn erc20_balance_of(contract_address: String, account: String) -> U256 {
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
    )
    .await
    .first()
    .unwrap()
    .clone() else {
        panic!("oops")
    };
    balance
}
