use candid::Nat;
use evm_rpc_canister_types::{
    EvmRpcCanister, GetTransactionCountArgs, GetTransactionCountResult,
    MultiGetTransactionCountResult, RpcServices,
};

/// Gets the transaction count of an account.
///
/// # Arguments
///
/// * `rpc_services` - The RPC services used to interact with the EVM.
/// * `get_transaction_count_args` - The arguments for getting the transaction count.
/// * `evm_rpc` - The EVM RPC canister used to send the transaction.
///
/// # Returns
///
/// The transaction count of the account.
///
/// # Panics
///
/// If the call fails on the system level, the responses are inconsistent or there is an RPC error.
pub async fn get_transaction_count(
    rpc_services: RpcServices,
    get_transaction_count_args: GetTransactionCountArgs,
    evm_rpc: EvmRpcCanister,
) -> Nat {
    let cycles = 10_000_000_000;
    let mut retry_counter = 0;

    loop {
        match evm_rpc
            .eth_get_transaction_count(
                rpc_services.clone(),
                None,
                get_transaction_count_args.clone(),
                cycles,
            )
            .await
            .expect("Call failed")
            .0
        {
            MultiGetTransactionCountResult::Consistent(r) => match r {
                GetTransactionCountResult::Ok(n) => break n,
                GetTransactionCountResult::Err(e) => {
                    ic_cdk::trap(format!("Error: {:?}", e).as_str())
                }
            },
            MultiGetTransactionCountResult::Inconsistent(_) => {
                if retry_counter == 3 {
                    ic_cdk::trap("Status is inconsistent");
                }
                retry_counter += 1;
                continue;
            }
        }
    }
}
