// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Script.sol";
import "../src/foundry/RecurringTransactions.sol";
import "../src/foundry/ERC20MintableByAnyone.sol";

contract MyScript is Script {
    function run(address chain_fusion_canister_address) external {
        // the private key of the deployer is the first private key printed by running anvil
        // hardhat account 0
        uint256 deployerPrivateKey = 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80;
        address deployerAddress = vm.addr(deployerPrivateKey); // 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
        console.log("Deployer address: ", deployerAddress);

        // hardhat account 1
        uint256 receiverPrivateKey = 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d;
        address receiverAddress = vm.addr(receiverPrivateKey); //0x70997970C51812dc3A010C7d01b50e0d17dc79C8;
        console.log("Receiver address: ", receiverAddress);

        // we use that key to broadcast all following transactions
        vm.startBroadcast(deployerPrivateKey);

        // this creates the contract. it will have the same address every time if we use a
        // new instance of anvil for every deployment.
        RecurringTransactions recurringTransactions = new RecurringTransactions(
            chain_fusion_canister_address
        );
        console.log(
            "RecurringTransactions address: ",
            address(recurringTransactions)
        );

        // deploy an erc20 token
        ERC20MintableByAnyone token = new ERC20MintableByAnyone(
            "Test Token",
            "TST"
        );

        console.log("Token address: ", address(token));

        // mint a bunch of tokens to the deployer
        uint256 aBunchOfTokens = 1e30;
        token.mint(address(deployerAddress), aBunchOfTokens);
        console.log(
            "Deployer address now has %s tokens",
            token.balanceOf(deployerAddress)
        );

        // approve the recurringTransactions contract to spend a bunch of tokens
        token.approve(address(recurringTransactions), aBunchOfTokens);
        console.log(
            "The recurringTransactions contract now has an approval of %s tokens",
            token.allowance(deployerAddress, address(recurringTransactions))
        );

        recurringTransactions.createJob{value: 0.1 ether}(
            240, // delay in seconds
            0.1 ether, // assuming 18 decimals here
            receiverAddress, // hardhat account 1
            address(token)
        );

        console.log("Job created");

        vm.stopBroadcast();
    }
}
