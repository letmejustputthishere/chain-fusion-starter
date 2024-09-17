use candid::Nat;
use evm_rpc_canister_types::{
    EvmRpcCanister, GetTransactionCountArgs, GetTransactionCountResult,
    MultiGetTransactionCountResult, RpcServices,
};

pub async fn get_transaction_count(
    rpc_services: RpcServices,
    get_transaction_count_args: GetTransactionCountArgs,
    evm_rpc: EvmRpcCanister,
) -> Nat {
    let cycles = 10_000_000_000;

    let (result,) = evm_rpc
        .eth_get_transaction_count(rpc_services, None, get_transaction_count_args, cycles)
        .await
        .expect("Call failed");

    match result {
        MultiGetTransactionCountResult::Consistent(r) => match r {
            GetTransactionCountResult::Ok(n) => n,
            GetTransactionCountResult::Err(e) => ic_cdk::trap(format!("Error: {:?}", e).as_str()),
        },
        MultiGetTransactionCountResult::Inconsistent(_) => {
            ic_cdk::trap("Status is inconsistent");
        }
    }
}
