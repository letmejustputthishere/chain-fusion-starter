# reTransICP: Recurring Transactions on Ethereum, powered by the Internet Computer

## What IS this?

This is a simple solution to a simple problem.
Smart contracts on Etherem Virtual Machines (EVMs) can not do anything without being triggered by an external entity. This leads to absurd situations, like Alice having to **manually** transfer ERC20 tokens to BOB each week. Like a peasant. This is not the future we were promised. This is not the future we want. This is not the future we deserve. This is not the future we will accept. This is not the future we will build. This is not the future we will live in. This is not the future we will die in. This is not the future we will be buried in. This is not the future we will be remembered in. This is not the future we will be forgotten ...

Anyway. We can use the Internet Computer to trigger an EVM smart contract to do something. And, even better, the Internet computer can listen to events the smart contract emits. This pretty much removes the necessity for a separate user interface, because now the smart contract can do order a wake-up call for tomorrow morning 10am. Actually 9am because it's a smart contract and it's always on time.

Check it out. It's pretty cool.

## How to use

### Prerequisites

Install dfx, rust, and caddy for local development. Installation involves the old `curl | sh` **ANTI-PATTERN**. Sorry about that, I did not create this, and at 2am in the morning I can't change it. But I totally understand if you don't want to run it. I didn't want to either. NOBODY WANTS TO RUN `curl | sh`, and for good reason.
Even worse, caddy wants to run as root. **TO INSTALL A \*\*\*\*\*\* ROOT CA CERTIFICATE**. No words here.

After that, try to calm down.

Then run `./deploy.sh` to get the party going. Read the script to learn more, it's pretty nice and well documented (credits go to Moritz Fuller).

### Developing

From there, adapting the code can be done within a reasonable amount of time. The rust code is in `canisters/` and the solidity code is in `contracts/`. The frontend is in `webapp/`.

#### Deploying the contract on an EVM

`forge script script/DeployContract.s.sol --private-key $PRIVATE_KEY --rpc-url $GNOSIS_RPC_URL --verify --verifier blockscout --verifier-url https://gnosis.blockscout.com/api? --broadcast`

To verify on another block explorer, use this:
`forge script script/DeployContract.s.sol --private-key $PRIVATE_KEY --rpc-url $GNOSIS_RPC_URL --verify --verifier-url https://api.gnosisscan.io/api\? --etherscan-api-key $GNOSISSCAN_API_KEY --resume`

Smart contract deployments:

- 0x5C5BB452Ffa9853B0688B3f45a6fF2113d2941fC
- https://gnosis.blockscout.com/address/0x5C5BB452Ffa9853B0688B3f45a6fF2113d2941fC
- https://gnosisscan.io/address/0x5C5BB452Ffa9853B0688B3f45a6fF2113d2941fC

#### Deploying the canister on the Internet Computer

Edit the `deploy_canister_to_production.sh` script to include your smart contracts address and topic hash.
Do this:

```
./deploy_canister_to_production.sh
dfx canister call chain_fusion_backend get_evm_address --ic
```

The source code of the contract can be found in `src/foundry/Coprocessor.sol`.

For local deployment of the EVM smart contract and submitting transactions, we use [foundry](https://github.com/foundry-rs/foundry). You can take a look at the steps needed to deploy the contract locally in the `deploy.sh` script which runs `script/Coprocessor.s.sol`. Make sure to check both files to understand the deployment process.

### chain fusion canister

The `chain_fusion_backend` canister listens to events emitted by the Ethereum smart contract by [periodically calling](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/advanced-features/periodic-tasks/#timers) the `eth_getLogs` RPC method via the [EVM RPC canister](https://github.com/internet-computer-protocol/evm-rpc-canister). When an event is received, the canister can do all kinds of synchronous and asynchronous processing. When the processing is done, the canister sends the results back by creating a transaction calling the `callback` function of the contract. The transaction is signed using a threshold signature and sent to the Ethereum network via the EVM RPC canister. You can learn more about how the EVM RPC canister works and how to integrate it [here](https://internetcomputer.org/docs/current/developer-docs/multi-chain/ethereum/evm-rpc/overview).

The logic for the job that is run on each event can be found in `src/chain_fusion_backend/job.rs`. The job is a simple example that just calculates Fibonacci numbers. You can replace this job with any other job you want to run on each event. The reason we picked this job is that it is computationally expensive and can be used to demonstrate the capabilities of the ICP as a coprocessor. Calculating the 20th fibonacci number wouldn't be possible on the EVM due to gas limits, but it is possible on the ICP.

```rust
pub async fn job(event_source: LogSource, event: LogEntry) {
    mutate_state(|s| s.record_processed_log(event_source.clone()));
    // because we deploy the canister with topics only matching
    // NewJob events we can safely assume that the event is a NewJob.
    let new_job_event = NewJobEvent::from(event);
    // this calculation would likely exceed an ethereum blocks gas limit
    // but can easily be calculated on the IC
    let result = fibonacci(20);
    // we write the result back to the evm smart contract, creating a signature
    // on the transaction with chain key ecdsa and sending it to the evm via the
    // evm rpc canister
    submit_result(result.to_string(), new_job_event.job_id).await;
    println!("Successfully ran job #{:?}", &new_job_event.job_id);
}
```

While not in use for this specific example, there is a `src/chain_fusion_backend/src/storage.rs` module that can be used to [write data to the canisters stable memory](https://github.com/dfinity/stable-structures/tree/main). This can be useful for storing big amounts of data (up to 400 GiB) in a canister. In our example, it can be used to store assets that are then served from the canister via [HTTP](https://internetcomputer.org/how-it-works/smart-contracts-serve-the-web/).

## Develop

The chain_fusion canister has been structured in a way that all the coprocessing logic lives in `src/chain_fusion_backend/src/job.rs` and developers don't need to recreate or touch the code responsible for fetching new events, creating signatures or sending transactions. They can solely focus on writing jobs to run upon receiving a new event from an EVM smart contract.

You can find the full flow in the following sequence diagram with Ethereum as an example EVM chain (note that this flow can be applied to any EVM chain):

![image](https://github.com/letmejustputthishere/chain-fusion-starter/assets/32162112/22272844-016c-43a0-a087-a861e930726c)

## chain_fusion starter use-cases

Here you can find a number of examples leveraging the chain_fusion starter logic:

- [On-chain asset and metadata creation for ERC721 NFT contracts](https://github.com/letmejustputthishere/chain-fusion-nft-creator)

Build your own use case on top of the chain_fusion starter and [share it with the community](https://github.com/letmejustputthishere/chain-fusion-starter/discussions/10)! Some ideas you could explore:

- A referral canister that distributes rewards to users based on their interactions with an EVM smart contract
- A ckNFT canister that mints an NFT on the ICP when an EVM helper smart contract emits a `ReceivedNft`, similar to the [`EthDepositHelper`](https://github.com/dfinity/ic/blob/master/rs/ethereum/cketh/minter/EthDepositHelper.sol) contract the ckETH minter uses. This could enable users to trade NFTs on the ICP without having to pay gas fees on Ethereum.
- Price oracles for DeFi applications via [exchange rate canister](https://github.com/dfinity/exchange-rate-canister)
- Prediction market resolution
- Soulbound NFT metadata and assets stored in a canister
- An on-chain managed passive index fund (e.g. top 10 ERC20 tokens traded on Uniswap)
- An on-chain donations stream
