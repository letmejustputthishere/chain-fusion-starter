# How was this library created?

-   used `didc` to generate the rust bindings from the evm rpc canister did file
-   added `call_with_payment128` to functions that expect cycles
-   derive `Debug` and `Clone` trait for types for convenience

# How to use this library?

-   make sure you deploy the evm rpc canister to its mainnet id locally (`7hfb6-caaaa-aaaar-qadga-cai`)
    ```json
    "evm_rpc": {
      "type": "custom",
      "candid": "https://github.com/internet-computer-protocol/evm-rpc-canister/releases/latest/download/evm_rpc.did",
      "wasm": "https://github.com/internet-computer-protocol/evm-rpc-canister/releases/latest/download/evm_rpc.wasm.gz",
      "remote": {
        "id": {
          "ic": "7hfb6-caaaa-aaaar-qadga-cai"
        }
      },
      "specified_id": "7hfb6-caaaa-aaaar-qadga-cai",
      "init_arg": "(record { nodesInSubnet = 28 })"
    }
    ```
-   if you deploy your own evm rpc canister, you can use the `Service` struct to initiate the canister with your own canister id
    ```rust
    pub const CANISTER_ID: Principal =
      Principal::from_slice(b"\x00\x00\x00\x00\x02\x30\x00\xCC\x01\x01"); // 7hfb6-caaaa-aaaar-qadga-cai
    pub const EVM_RPC: Service = Service(CANISTER_ID);
    ```
-   import the libary in your rust project
    ```toml
    [dependencies]
    evm_rpc_canister_types = 0.1.0
    ```
-   import the crate where needed, e.g.
    ```rust
    use evm_rpc_canister_types::{
      BlockTag, GetBlockByNumberResult, GetLogsArgs, GetLogsResult, HttpOutcallError,
      MultiGetBlockByNumberResult, MultiGetLogsResult, RejectionCode, RpcError, EVM_RPC,
    };
    ```
-   the `EVM_RPC` struct exposes the EVM RPC canisters interface and is used to make inter canister calls to it
    ```rust
    let (result,) = EVM_RPC
      .eth_get_block_by_number(rpc_providers, None, block_tag, cycles)
      .await
      .expect("Call failed");
    ```
