use evm_rpc_canister_types::{EvmRpcCanister, RequestResult, RpcService};
use serde::{Deserialize, Serialize};

use crate::request_cost::request_cost;

pub async fn request(
    rpc_service: RpcService,
    json_rpc_payload: String,
    max_response_bytes: u64,
    evm_rpc: EvmRpcCanister,
) -> RequestResult {
    // estimate cycles costs
    let cycles = request_cost(
        rpc_service.clone(),
        json_rpc_payload.clone(),
        max_response_bytes,
        evm_rpc.clone(),
    )
    .await;
    // call request with estimated cycles
    match evm_rpc
        .request(rpc_service, json_rpc_payload, max_response_bytes, cycles)
        .await
    {
        Ok((res,)) => res,
        Err(e) => ic_cdk::trap(format!("Error: {:?}", e).as_str()),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonRpcResult {
    pub result: Option<String>,
    pub error: Option<JsonRpcError>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: isize,
    pub message: String,
}
