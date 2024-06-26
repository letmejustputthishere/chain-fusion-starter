# Why was this library created

-   there are some common patterns when interacting with EVM smart contracts from the IC via the EVM RPC canister that can be abstracted away from developers, this helps reduce writing boilerplate code and makes it easier to interact with the EVM RPC canister
-   the library provides a set of types and functions that can be used to interact with the EVM
    -   `evm_signer`: a module that provides a way to sign messages using the t-ECDSA and get the public key and EVM address of the signer
    -   `fees`: a module that provides a way to calculate the fees for a given transaction
    -   `conversions`: some helpful functions to convert between different types commonly used by the ethers crate
    -   `eth_call`: a module that provides a way to call a smart contract function without modifying the state of the EVM, this is useful for reading data from the EVM and achieved by calling the `request` EVM RPC function
    -   includes `erc20_balance_of` built on top of `eth_call` to get the balance of an ERC20 token
    -   `eth_send_raw_transaction`: a module that provides a way to send a signed transaction to the EVM, this is useful for modifying the state of the EVM and achieved by calling the `send_raw_transaction` EVM RPC function
    -   includes `transfer_eth` and `contract_interaction` functions built on top of `eth_send_raw_transaction` to send ETH and interact with smart contracts
    -   `request`: a module that provides a way to make arbitrary RPC requests, includes determening the cycles costs of the request
    -   `request_costs`: a module that provides a way to calculate the cycles costs of a given RPC request

# How to use this library?

-   the methods in this crate rely on the `EvmRpcCanister` struct to make inter canister calls to the EVM RPC canister, this struct is used to initiate calls to the EVM RPC Canister
    -   you can use the [`evm-rpc-canister-types`](https://crates.io/crates/evm-rpc-canister-types) crate to create this struct
-   import the libary in your rust project
    ```toml
    [dependencies]
    ic-evm-utils= 0.1
    ```
-   import the crate where needed and pass the `EvmRpcCanister` struct to the functions if necessary
    ```rust
    use ic_evm_utils::eth_send_raw_transaction::{contract_interaction, ContractDetails};
    // ...
    let status = contract_interaction(
        contract_details,
        gas,
        rpc_services,
        nonce,
        key_id,
        vec![],
        EVM_RPC, // EvmRpcCanister struct
    )
    .await;
    ```
