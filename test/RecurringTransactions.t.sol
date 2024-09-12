// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.20;

import "forge-std/Test.sol";
import "../contracts/RecurringTransactions.sol";
import "../contracts/ERC20MintableByAnyone.sol";

contract RecurringTransactionsTest is Test {
    function testExecuteJob() public {
        address trigger = address(123);
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
        uint128 amountToTransfer = 0.3 ether;

        // Give the sender 1 ETH
        vm.deal(sender, 1 ether);

        // store initial balances
        uint256 initialRecipientTokenBalance = token.balanceOf(recipient);

        vm.startPrank(sender);
        token.approve(address(recurringTransactions), aBunchOfTokens);
        recurringTransactions.createJob{value: 0.1 ether}(
            20,
            10,
            amountToTransfer,
            recipient,
            address(token)
        );
        vm.stopPrank();

        // Check if tokens were transferred to the recipient
        assertEq(
            token.balanceOf(recipient) - initialRecipientTokenBalance,
            amountToTransfer,
            "Incorrect amount of tokens transferred to recipient"
        );

        // Get the job details
        (
            uint64 storedPeriod,
            uint64 storedNumberOfRemainingExecutions,
            uint128 storedAmount,
            uint256 storedLastExecutionTime,
            address storedSender,
            address storedRecipient,
            address storedTokenAddress
        ) = recurringTransactions.jobs(0);

        // Check if job details are correctly stored
        assertEq(storedPeriod, 20, "Incorrect delay stored");
        assertEq(
            storedNumberOfRemainingExecutions,
            9,
            "Incorrect remaining executions stored"
        );
        assertEq(storedAmount, amountToTransfer, "Incorrect amount stored");
        assertEq(storedSender, sender, "Incorrect sender stored");
        console.log("jobRecipient", storedRecipient);
        console.log("recipient", recipient);
        assertEq(storedRecipient, recipient, "Incorrect recipient stored");
        assertEq(
            storedTokenAddress,
            address(token),
            "Incorrect token address stored"
        );
        assertEq(
            storedLastExecutionTime,
            1,
            "Incorrect last execution time stored"
        );

        // Check if ETH was sent to the executor (trigger)
        assertEq(
            trigger.balance,
            0.1 ether,
            "Incorrect amount of ETH transferred to executor"
        );
    }

    function testRemainingExecutions() public {
        address trigger = address(123);
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
        uint128 amountToTransfer = 0.3 ether;

        vm.deal(sender, 2 ether);

        vm.startPrank(sender);
        token.approve(address(recurringTransactions), aBunchOfTokens);

        // Create a job with finite executions
        recurringTransactions.createJob{value: 0.1 ether}(
            20,
            10,
            amountToTransfer,
            recipient,
            address(token)
        );

        vm.stopPrank();

        // Execute the finite job multiple times
        for (uint i = 0; i < 6; i++) {
            vm.warp((i + 1) * 25);
            vm.prank(trigger);
            recurringTransactions.executeJob(0);
        }

        // Check remaining executions for finite job
        (, uint64 remainingFinite, , , , , ) = recurringTransactions.jobs(0);
        assertEq(
            remainingFinite,
            3,
            "Incorrect remaining executions for finite job"
        );

        // Execute the finite job multiple times
        for (uint i = 5; i < 8; i++) {
            vm.warp((i + 1) * 25);
            vm.prank(trigger);
            recurringTransactions.executeJob(0);
        }

        (, remainingFinite, , , , , ) = recurringTransactions.jobs(0);
        assertEq(
            remainingFinite,
            0,
            "Incorrect remaining executions for finite job"
        );

        // ensure next execution reverts
        vm.warp(10000);
        vm.prank(trigger);
        vm.expectRevert();
        recurringTransactions.executeJob(0);
    }
}
