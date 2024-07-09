//! This module provides functions for estimating the cycles cost of a call made to the EVM RPC canister's `request` method.
use evm_rpc_canister_types::{EvmRpcCanister, RequestCostResult, RpcService};

use crate::conversions::nat_to_u128;

/// Provides the cycles cost of a call made to the EVM RPC canister's `request` method.
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
/// The cycles cost of the call.
pub async fn request_cost(
    rpc_service: RpcService,
    json_rpc_payload: String,
    max_response_bytes: u64,
    evm_rpc: EvmRpcCanister,
) -> u128 {
    // Get cycles cost
    let cycles_result = match evm_rpc
        .request_cost(rpc_service, json_rpc_payload, max_response_bytes)
        .await
    {
        Ok((res,)) => res,
        Err(e) => ic_cdk::trap(format!("Error: {:?}", e).as_str()),
    };

    // trap if there is an rpc error
    match cycles_result {
        RequestCostResult::Ok(cycles) => nat_to_u128(&cycles),
        RequestCostResult::Err(e) => ic_cdk::trap(&format!("error in `request_cost`: {:?}", e)),
    }
}
