use candid::Nat;
use serde_bytes::ByteBuf;

use crate::{
    evm_rpc::{
        BlockTag, FeeHistory, FeeHistoryArgs, FeeHistoryResult, MultiFeeHistoryResult, EVM_RPC,
    },
    state::read_state,
};

pub async fn fee_history(
    block_count: Nat,
    newest_block: BlockTag,
    reward_percentiles: Option<ByteBuf>,
) -> FeeHistory {
    let rpc_providers = read_state(|s| s.rpc_services.clone());
    let fee_history_args: FeeHistoryArgs = FeeHistoryArgs {
        blockCount: block_count,
        newestBlock: newest_block,
        rewardPercentiles: reward_percentiles,
    };

    let cycles = 10_000_000_000;

    match EVM_RPC
        .eth_fee_history(rpc_providers, None, fee_history_args, cycles)
        .await
    {
        Ok((res,)) => match res {
            MultiFeeHistoryResult::Consistent(fee_history) => match fee_history {
                FeeHistoryResult::Ok(fee_history) => fee_history.unwrap(),
                FeeHistoryResult::Err(e) => {
                    ic_cdk::trap(format!("Error: {:?}", e).as_str());
                }
            },
            MultiFeeHistoryResult::Inconsistent(_) => {
                ic_cdk::trap("Fee history is inconsistent");
            }
        },
        Err(e) => ic_cdk::trap(format!("Error: {:?}", e).as_str()),
    }
}
