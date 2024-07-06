#!/bin/bash

dfx stop
# Find process IDs listening on port 4943 (dfx)
dfx=$(lsof -t -i:4943)
# Check if any PIDs were found
if [ -z "$dfx" ]; then
    echo "dfx not running."
else
    # Kill the processes
    kill $dfx && echo "Terminating running dfx instance."
    sleep 3
fi
# dfx start --clean --background
# dfx ledger fabricate-cycles --icp 10000 --canister $(dfx identity get-wallet)
# dfx deploy evm_rpc
cargo build --release --target wasm32-unknown-unknown --package chain_fusion_backend
dfx canister create --with-cycles 10_000_000_000_000 chain_fusion_backend --ic
# because the local smart contract deployment is deterministic, we can hardcode the 
# the `get_logs_address` here. in our case we are listening for NextExecutionTimestamp events,
# you can read more about event signatures [here](https://docs.alchemy.com/docs/deep-dive-into-eth_getlogs#what-are-event-signatures)
# (we can use cast sig-event "NextExecutionTimestamp(uint, uint indexed)" to get the topic)
dfx canister install --mode reinstall --ic --wasm target/wasm32-unknown-unknown/release/chain_fusion_backend.wasm chain_fusion_backend --argument '(
  record {
    ecdsa_key_id = record {
      name = "key_1";
      curve = variant { secp256k1 };
    };
    get_logs_topics = opt vec {
      vec {
        "0xd270de418848f07676c092e30c67a99070a18f01b8f573731322eadeea0c1ab8";
      };
    };
    last_scraped_block_number = 34832736: nat;
    rpc_services = variant {
      Custom = record {
        chainId = 100 : nat64;
        services = vec { record { url = "https://rpc.ankr.com/gnosis"; headers = null } };
      }
    };
    rpc_service = variant {
      Custom = record {
        url = "https://rpc.ankr.com/gnosis";
        headers = null;
      }
    };
    get_logs_address = vec { "0x146f441905723173b4d6ca06e9Fa911b3B707074" };
    block_tag = variant { Latest = null };
    nonce = 1 : nat;
  },
)'
# beware: the nonce above is not used yet. Instead, it is hardcoded in lifecycle.rs

# sleep for 3 seconds to allow the evm address to be generated
sleep 3
# save the chain_fusion canisters evm address
export EVM_ADDRESS=$(dfx canister call chain_fusion_backend get_evm_address --ic | awk -F'"' '{print $2}')

# smart contract has already been deployed and needs manual setup with the evm address now