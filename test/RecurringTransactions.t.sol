// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "../contracts/RecurringTransactions.sol";
import "../contracts/ERC20MintableByAnyone.sol";

contract RecurringTransactionsTest is Test {
    function testExecuteJob() public {
        address trigger = address(1);
        address sender = address(2);
        address recipient = address(3);

        ERC20MintableByAnyone token = new ERC20MintableByAnyone(
            "Test Token",
            "TST"
        );

        RecurringTransactions recurringTransactions = new RecurringTransactions(
            trigger
        );

        uint256 aBunchOfTokens = 1e30;
        token.mint(sender, aBunchOfTokens);

        vm.startPrank(sender);
        token.approve(address(recurringTransactions), aBunchOfTokens);
        recurringTransactions.createJob(
            20,
            10,
            0.3 ether,
            recipient,
            address(token)
        );
        vm.stopPrank();

        console.log(
            "RecurringTransactions address: ",
            address(recurringTransactions)
        );

        vm.stopBroadcast();
    }
}
