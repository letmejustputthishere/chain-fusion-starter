// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Script.sol";
import "../src/foundry/RecurringTransactions.sol";
import "../src/foundry/ERC20MintableByAnyone.sol";

contract MyScript is Script {
    function run(address chain_fusion_canister_address) external {
        // the private key of the deployer is the first private key printed by running anvil
        uint256 deployerPrivateKey = 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80;
        // we use that key to broadcast all following transactions
        vm.startBroadcast(deployerPrivateKey);

        // this creates the contract. it will have the same address every time if we use a
        // new instance of anvil for every deployment.

        RecurringTransactions recurringTransactions = new RecurringTransactions(
            chain_fusion_canister_address
        );

        // deploy an erc20 token
        ERC20MintableByAnyone token = new ERC20MintableByAnyone(
            "Test Token",
            "TST"
        );

        // mint 1000 tokens to the deployer
        token.mint(address(this), 1000);

        // approve the recurringTransactions contract to spend a bunch of tokens
        token.approve(address(recurringTransactions), 10000e18);

        // create a job that will send 0,3 tokens to the deployer every 20 seconds
        recurringTransactions.createJob(
            20,
            0.3 ether, // assuming 18 decimals here
            address(2), // some memorable address
            address(token)
        );

        vm.stopBroadcast();
    }
}
