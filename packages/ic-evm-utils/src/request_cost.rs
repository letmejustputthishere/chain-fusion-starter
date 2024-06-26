use evm_rpc_canister_types::{RequestCostResult, RpcService, EvmRpcCanister};

use crate::conversions::nat_to_u128;

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
