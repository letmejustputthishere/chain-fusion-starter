use candid::Nat;
use ethers_core::types::U256;
use serde_bytes::ByteBuf;
use std::ops::Add;

use crate::{
    evm_rpc::{
        BlockTag, FeeHistory, FeeHistoryArgs, FeeHistoryResult, MultiFeeHistoryResult, EVM_RPC,
    },
    state::read_state,
    utils,
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

pub async fn estimate_transaction_fees() -> (U256, U256) {
    // we are setting the `max_priority_fee_per_gas` based on this article:
    // https://docs.alchemy.com/docs/maxpriorityfeepergas-vs-maxfeepergas
    // following this logic, the base fee will be derived from the block history automatically
    // and we only specify the maximum priority fee per gas (tip).
    // the tip is derived from the fee history of the last 9 blocks, more specifically
    // from the 95th percentile of the tip.
    let fee_history = fee_history(
        Nat::from(9u32),
        BlockTag::Latest,
        Some(ByteBuf::from(vec![95])),
    )
    .await;

    // baseFeePerGas median over the past 9 blocks
    let mut base_fee_per_gas = fee_history.baseFeePerGas;
    // sort the base fees in ascending order
    base_fee_per_gas.sort_unstable();
    // get the median by accessing the element in the middle
    let base_fee = base_fee_per_gas
        .get(4)
        .expect("the base_fee_per_gas should have 9 elements")
        .clone();

    // obtain the 95th percentile of the tips for the past 9 blocks
    let mut percentile_95: Vec<Nat> = fee_history
        .reward
        .into_iter()
        .flat_map(|x| x.into_iter())
        .collect();
    // sort the tips in ascending order
    percentile_95.sort_unstable();
    // get the median by accessing the element in the middle
    let max_priority_fee_per_gas = percentile_95
        .get(4)
        .expect("the 95th percentile should have 9 elements")
        .clone();

    let max_fee_per_gas = max_priority_fee_per_gas.clone().add(base_fee);

    (
        utils::nat_to_u256(&max_fee_per_gas),
        utils::nat_to_u256(&max_priority_fee_per_gas),
    )
}
