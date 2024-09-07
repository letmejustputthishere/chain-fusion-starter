// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Script.sol";
import "../contracts/RecurringTransactions.sol";
import "../contracts/ERC20MintableByAnyone.sol";

contract MyScript is Script {
    function run() external {
        vm.startBroadcast();

        // this creates the contract. it will have the same address every time if we use a
        // new instance of anvil for every deployment.
        RecurringTransactions recurringTransactions = new RecurringTransactions(
            address(0)
        );

        console.log(
            "RecurringTransactions address: ",
            address(recurringTransactions)
        );

        vm.stopBroadcast();
    }
}
