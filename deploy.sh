# Checking if any process is listening on port 8545 (default for anvil)
if lsof -i :8545 | grep -q ":8545 "; then
    echo "Anvil is running."
else
    echo "Anvil is not running."
    exit 1  # Exit with a status code of 1, indicating an error
fi
# deploy the contract and mint one nft
forge script script/NFT.s.sol:MyScript --fork-url http://localhost:8545 --broadcast
dfx stop
dfx start --clean --background
dfx ledger fabricate-cycles --icp 10000 --canister $(dfx identity get-wallet)
dfx deps pull                                                              
dfx deps init evm_rpc --argument '(record { nodesInSubnet = 28 })'
dfx deps deploy
# because our local NFT contract deployment is deterministic, we can hardcode the 
# the `get_logs_address` here. in our case we are listening for mint events,
# that is transfer events with the `from` field being the zero address.
# you can read more about event signatures [here](https://docs.alchemy.com/docs/deep-dive-into-eth_getlogs#what-are-event-signatures)
dfx deploy dappcon_backend --with-cycles 10_000_000_000_000 --argument '(
  record {
    ecdsa_key_id = record {
      name = "dfx_test_key";
      curve = variant { secp256k1 };
    };
    get_logs_topics = opt vec {
      vec {
        "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";
      };
      vec {
        "0x0000000000000000000000000000000000000000000000000000000000000000";
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