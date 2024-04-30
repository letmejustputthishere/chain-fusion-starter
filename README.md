# [Chainfusion](https://internetcomputer.org/chainfusion) starter

## Join the discussion

[![Join Discussion](https://img.shields.io/github/discussions/letmejustputthishere/chainfusion-starter)](https://github.com/letmejustputthishere/chainfusion-starter/discussions)

## Get started:

No matter what setup you pick from below, run `./deploys.sh` from the project root to deploy the project. To understand the steps involved in deploying the project locally, examine the comments in `deploy.sh`. This script will

-   start anvil
-   start dfx
-   deploy the EVM contract
-   generate a number of jobs
-   deploy the chainfusion canister

If you want to check that the `chainfusion_backend` really processed the events, you can either look at the logs output by running `./deploy.sh` – keep an eye open for the `Successfully ran job` message – or you can call the EVM contract to get the results of the jobs.
To do this, run `cast call --rpc-url=127.0.0.1:8545 0x5fbdb2315678afecb367f032d93f642f64180aa3  "getResult(uint)(string)" <job_id>` where `<job_id>` is the id of the job you want to get the result for. This should always return `"6765"` for processed jobs, which is the 20th fibonacci number, and `""` for unprocessed jobs.

If you want to create more jobs, simply run `cast send --rpc-url=127.0.0.1:8545 0x5fbdb2315678afecb367f032d93f642f64180aa3  "newJob()" --private-key=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --value 0.01ether`.

You can learn more about how to use cast [here](https://book.getfoundry.sh/reference/cast/).

### In the cloud:

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/letmejustputthishere/chainfusion-starter/?quickstart=1)

### Locally:

Make sure you have you have Docker and VS Code installed and running, then click the button below

[![Open locally in Dev Containers](https://img.shields.io/static/v1?label=Dev%20Containers&message=Open&color=blue&logo=visualstudiocode)](https://vscode.dev/redirect?url=vscode://ms-vscode-remote.remote-containers/cloneInVolume?url=https://github.com/letmejustputthishere/chainfusion-starter)

### Or do the manual setup:

Make sure that [Node.js](https://nodejs.org/en/) `>= 21`, [foundry](https://github.com/foundry-rs/foundry), [caddy](https://caddyserver.com/docs/install#install) and [dfx](https://internetcomputer.org/docs/current/developer-docs/build/install-upgrade-remove) `>= 0.18` are installed on your system.

Run the following commands in a new, empty project directory:

```sh
git clone https://github.com/letmejustputthishere/chainfusion-starter.git # Download this starter project
cd chainfusion-starter # Navigate to the project directory
```

## Overview

This project demonstrates how to use the Internet Computer as a coprocessor for EVM smart contracts. The coprocessor listens to events emitted by an EVM smart contract, processes them, and optionally sends the results back. Note that way say EVM smart contracts, as you can not only interact with the Ethereum network, but other networks that are using the Ethereum Virtual Machine (EVM), such as Polygon and Avalanche.

This is an early project and should be considered as a proof of concept. It is not production-ready and should not be used in production environments. There are quite some TODOs in the code which will be addressed over time. If you have any questions or suggestions, feel free to open an issue, start a [discussion](https://github.com/letmejustputthishere/chainfusion-starter/discussions) or reach out to me on the [DFINITY Developer Forum](https://forum.dfinity.org/u/cryptoschindler/summary) or [X](https://twitter.com/cryptoschindler).

## What is a coprocessor?

The concept of coprocessors originated in computer architecture as a technique to enhance performance. Traditional computers rely on a single central processing unit (CPU) to handle all computations. However, the CPU became overloaded as workloads grew more complex.

Coprocessors were introduced to offload specific tasks from the CPU to specialized hardware. We see the same happening in the EVM ecosystem. EVM smart contracts, and Ethereum in particular, are a very constrained computing environment. Coprocessors and stateful Layer 2 solutions enable to extend the capabilities of the EVM by offloading specific tasks to more powerful environments.

You can read more about coprocessors in the context of Ethereum in the article ["A Brief Into to Coprocessors"](https://crypto.mirror.xyz/BFqUfBNVZrqYau3Vz9WJ-BACw5FT3W30iUX3mPlKxtA). The first paragraph of this section was directly taken from this article.

## Why use ICP as a coprocessor for Ethereum?

Canister smart contracts on ICP can securely read from EVM smart contracts (using [HTTPS Outcalls](https://internetcomputer.org/https-outcalls) or the [EVM RPC](https://internetcomputer.org/docs/current/developer-docs/multi-chain/ethereum/evm-rpc/overview) canister) and write to them (using Chain-key Signatures, i.e. [Threshold ECDSA](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/encryption/t-ecdsa)). Hence, there are no additional parties needed to relay messages between the two networks, and no additional work needs to be done on the EVM side to verify the results of the computation as the EVM smart contract just needs to check for the proper sender.

Furthermore, canister smart contracts have many capabilities and properties that can be leveraged to extend the reach of smart contracts:

-   WASM Runtime, which is much more efficient than the EVM, and allows programming in [Rust, JavaScript, and other traditional languages](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/write/overview#choosing-the-programming-language-for-the-backend) (next to [Motoko](https://internetcomputer.org/docs/current/motoko/main/motoko/)).
-   [400 GiB of memory](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/best-practices/storage/) with the cost of storing 1 GiB on-chain for a year only being $5
-   [Long-running computations](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/maintain/resource-limits/) that even allow [running AI inference](https://x.com/dominic_w/status/1770884845570326589).
-   [HTTPS Outcalls](https://internetcomputer.org/docs/current/references/https-outcalls-how-it-works) allow canisters to interact with other chains and traditional web services.
-   [Chain-key signatures](https://internetcomputer.org/docs/current/references/t-ecdsa-how-it-works) allow canisters to sign transactions for other chains, including Ethereum and Bitcoin.
-   [Timers](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/advanced-features/periodic-tasks/) allow syncing with EVM events and scheduling other tasks.
-   [Unbiasable randomness](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/advanced-features/randomness/) provided by the threshold BLS signatures straight from the heart of [ICP's Chain-key technology](https://internetcomputer.org/how-it-works/chain-key-technology/).
-   [Serve webcontent](https://internetcomputer.org/how-it-works/smart-contracts-serve-the-web/) directly from canisters via the [HTTP gateway protocol](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/advanced-features/serving-http-request)
-   The [reverse gas model](https://internetcomputer.org/docs/current/developer-docs/gas-cost/#the-reverse-gas-model) frees end users from paying for every transaction they perform
-   ~1-2 second [finality](https://internetcomputer.org/how-it-works/consensus/)
-   [Multi-block transactions](https://internetcomputer.org/capabilities/multi-block-transactions/)

For more context on how ICP can extend Ethereum, check out [this presentation](https://docs.google.com/presentation/d/1P9wycxRsJ6DM_c8TbZG4Xun5URYZbk3WALS4UpSH0iA/edit?usp=sharing) from EthereumZuri 2024.

## Architecture

![image](https://github.com/letmejustputthishere/chainfusion-starter/assets/32162112/7947d2f1-bbaa-4291-b089-2eb05c5d42df)

### EVM Smart contract

The contract `Coprocessor.sol` emits an event `NewJob` when the `newJob` function is called. The `newJob` function transfers the ETH sent with the call to `newJob` to the account controlled by the `chainfusion_backend` canister and emits the event. We send ETH to the `chainfusion_backend` canister to pay for the processing of the job result and the transaction fees for sending the result back to the EVM smart contract.

```solidity
    function newJob() public payable {
        // Require at least 0.01 ETH to be sent with the call
        require(msg.value >= 0.01 ether, "Minimum 0.01 ETH not met");

        // Forward the ETH received to the coprocessor address
        // To pay for the submission of the job result back to the EVM
        // contract.
        (bool success, ) = coprocessor.call{value: msg.value}("");
        require(success, "Failed to send Ether");

        // Emit the new job event
        emit NewJob(job_id);

        // Increment job counter
        job_id++;
    }
```

The contract also has a `callback` function that can only be called by the `chainfusion_backend` canister. This function is called by the `chainfusion_backend` canister to send the results of the processing back to the contract.

```solidity
    function callback(string calldata _result, uint256 _job_id) public {
        require(
            msg.sender == coprocessor,
            "Only the coprocessor can call this function"
        );
        jobs[_job_id] = _result;
    }
```

The source code of the contract can be found in `src/foundry/Coprocessor.sol`.

For local deployment of the EVM smart contract and submitting transactions we use [foundry](https://github.com/foundry-rs/foundry). You can take a look at the steps needed to deploy the contract locally in the `deploy.sh` script which runs `script/Coprocessor.s.sol`. Make sure to check both files to understand the deployment process.

### Chainfusion canister

The `chainfusion_backend` canister listens to events emitted by the Ethereum smart contract by periodically calling the `eth_getLogs` RPC method via the [EVM RPC canister](https://github.com/internet-computer-protocol/evm-rpc-canister). When an event is received, the canister can do all kinds of synchronous and asynchronous processing. When the processing is done, the canister sends the results back by creating a transaction calling the `callback` function of the contract. The transaction is signed using threshold signatures and sent to the Ethereum network via the EVM RPC canister. You can learn more about how the EVM RPC canister works and how to integrate with it [here](https://internetcomputer.org/docs/current/developer-docs/multi-chain/ethereum/evm-rpc/overview).

The logic for the job that is run on each event can be found in `src/chainfusion_backend/job.rs`. The job is a simple example that just calculates fibonacci numbers. You can replace this job with any other job you want to run on each event. The reason we picked this job is that it is computationally expensive and can be used to demonstrate the capabilities of the ICP as a coprocessor. Calculating the 20th fibonacci number wouldn't be possible on the EVM due to gas limits, but it is possible on the ICP.

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

## Develop

The Chainfusion canister has been structured in a way that all the coprocessing logic lives in `src/chainfusion_backend/src/job.rs` and developers don't need to recreate or touch the code responsible for fetching new events, creating signatures or sending transactions. They can solely focus on writing jobs to run upon receiving a new event from an EVM smart contract.

You can find the full flow in the following sequence diagram with Ethereum as an example EVM chain (note that this flow can be applied to any EVM chain):

![image](https://github.com/letmejustputthishere/chainfusion-starter/assets/32162112/22272844-016c-43a0-a087-a861e930726c)

## Chainfusion starter use-cases

Here you can find a number of examples leveraging the Chainfusion starter logic:

-   On-chain asset and metadata creation for ERC721 NFT contracts

Build your own use-case on top of the Chainfusion starter and [share it with the community](https://github.com/letmejustputthishere/chainfusion-starter/discussions/10)! Some ideas you could explore:

-   A referral canister that distributes rewards to users based on their interactions with an EVM smart contract
-   A ckNFT canister that mints an NFT on the ICP when an EVM helper smart contract emits an `ReceivedNft`, similar to the [`EthDepositHelper`](https://github.com/dfinity/ic/blob/master/rs/ethereum/cketh/minter/EthDepositHelper.sol) contract the ckETH minter uses. This could enable users to trade NFTs on the ICP without having to pay gas fees on Ethereum.
