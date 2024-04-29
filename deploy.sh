#!/bin/bash

# Find process IDs listening on port 8545 (anvil)
anvil=$(lsof -t -i:8545)

# Check if any PIDs were found
if [ -z "$anvil" ]; then
    echo "Anvil not running."
else
    # Kill the processes
    kill $anvil && echo "Terminated running Anvil process."
    sleep 3
fi

# start anvil with slots in an epoch send to 1 for faster finalised blocks
anvil --slots-in-an-epoch 1 &
# kill caddyserver
caddy stop
# start caddyserver
caddy start
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
dfx start --clean --background
dfx ledger fabricate-cycles --icp 10000 --canister $(dfx identity get-wallet)
dfx deploy evm_rpc
# because our local NFT contract deployment is deterministic, we can hardcode the 
# the `get_logs_address` here. in our case we are listening for mint events,
# that is transfer events with the `from` field being the zero address.
# you can read more about event signatures [here](https://docs.alchemy.com/docs/deep-dive-into-eth_getlogs#what-are-event-signatures)
dfx deploy chainfusion_backend --with-cycles 10_000_000_000_000 --argument '(
  record {
    ecdsa_key_id = record {
      name = "dfx_test_key";
      curve = variant { secp256k1 };
    };
    get_logs_topics = opt vec {
      vec {
        "0x031ada964b8e520743eb9508d0ace62654b126430b7e5a92b42e78eebb61602e";
      };
    };
    last_scraped_block_number = 0: nat;
    rpc_services = variant {
      Custom = record {
        chainId = 31_337 : nat64;
        services = vec { record { url = "https://localhost:8546"; headers = null } };
      }
    };
    get_logs_address = vec { "0x5FbDB2315678afecb367f032d93F642f64180aa3" };
    block_tag = variant { Latest = null };
  },
)'
# sleep for 3 seconds to allow the evm address to be generated
sleep 3
# safe the chainfusion canisters evm address
export EVM_ADDRESS=$(dfx canister call chainfusion_backend get_evm_address | awk -F'"' '{print $2}')
# deploy the contract passing the chainfusion canisters evm address to receive the fees and create a couple of new jobs
forge script script/Coprocessor.s.sol:MyScript --fork-url http://localhost:8545 --broadcast --sig "run(address)" $EVM_ADDRESS