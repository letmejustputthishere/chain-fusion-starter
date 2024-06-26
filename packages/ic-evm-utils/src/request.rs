//! This module provides functions for making arbitrary requests to EVM RPC providers through the EVM RPC canister.
use evm_rpc_canister_types::{EvmRpcCanister, RequestResult, RpcService};
use serde::{Deserialize, Serialize};

use crate::request_cost::request_cost;

/// Make a arbitrary request to EVM RPC provider through the EVM RPC canister.
///
/// # Arguments
///
/// * `rpc_service` - The RPC service used to interact with the EVM.
/// * `json_rpc_payload` - The JSON-RPC payload to send.
/// * `max_response_bytes` - The maximum number of response bytes to accept.
/// * `evm_rpc` - The EVM RPC canister.
///
/// # Returns
///
/// The result of the request.
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

/// Represents a JSON-RPC result.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonRpcResult {
    pub result: Option<String>,
    pub error: Option<JsonRpcError>,
}

/// Represents a JSON-RPC error.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: isize,
    pub message: String,
}
