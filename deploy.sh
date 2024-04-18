network=${1:-local}

dfx stop
dfx start --clean --background
dfx ledger fabricate-cycles --icp 10000 --canister $(dfx identity get-wallet)
dfx deps pull                                                              
dfx deps init evm_rpc --argument '(record { nodesInSubnet = 28 })'
dfx deps deploy
dfx deploy dappcon_backend --with-cycles 10_000_000_000_000  --network $network --argument '(
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
        "0x00000000000000000000000000b46c2526e227482e2ebb8f4c69e4674d262e75";
      };
      vec {
        "0x00000000000000000000000054a2d42a40f51259dedd1978f6c118a0f0eff078";
      };
    };
    last_scraped_block_number = 4_365_620 : nat;
    rpc_services = variant { EthSepolia = null };
    get_logs_address = vec { "0xb59f67a8bff5d8cd03f6ac17265c550ed8f33907" };
  },
)'