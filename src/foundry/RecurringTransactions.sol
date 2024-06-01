// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

struct job {
    uint256 period;
    uint256 amount;
    uint256 lastExecution;
    address sender;
    address recipient;
    address token;
}

contract RecurringTransactions {
    job[] public jobs;

    address payable private immutable trigger;

    constructor(address _coprocessor) {
        trigger = payable(_coprocessor);
    }

    event NextExecutionTimestamp(uint date, uint indexed job_id);

    function transferToken(
        address _token,
        address _sender,
        address _recipient,
        uint256 _amount
    ) public {
        IERC20 token = IERC20(_token);
        SafeERC20.safeTransferFrom(token, _sender, _recipient, _amount);
    }

    // Function to create a new job
    function createJob(
        uint256 period,
        uint256 amount,
        address sender,
        address recipient,
        address token
    ) public payable {
        // Require at least 0.01 ETH to be sent with the call
        require(msg.value >= 0.01 ether, "Minimum 0.01 ETH not met");

        // store the job details
        uint256 job_id = jobs.length;
        jobs.push(
            job(period, amount, block.timestamp, msg.sender, recipient, token)
        );

        // Forward the ETH received to the coprocessor address
        // to pay for contract executions
        trigger.transfer(msg.value);

        // transfer the token to the recipient
        transferToken(token, msg.sender, recipient, amount);

        // order next execution
        emit NextExecutionTimestamp(block.timestamp + period, job_id);
    }

    function removeJob(uint _job_id) public returns (string memory) {
        job memory _job = jobs[_job_id];

        require(
            msg.sender == _job.sender,
            "Only the sender can remove the job"
        );
        // make sure the job exists
        require(_job_id < jobs.length, "Job does not exist");

        // remove the job
        delete jobs[_job_id];
        return "Job removed";
    }

    function executeJob(uint256 _job_id) public {
        // todo: actually, anyone can call this function
        require(
            msg.sender == trigger,
            "Only the coprocessor can call this function"
        );
        // make sure the job exists
        job memory _job = jobs[_job_id];
        require(_job.sender != address(0), "Job does not exist");

        // check if the job is due
        require(
            block.timestamp >= _job.lastExecution + _job.period,
            "Job not due yet"
        );

        // transfer the token to the recipient
        transferToken(_job.token, _job.sender, _job.recipient, _job.amount);

        // update the last execution timestamp. We assume perfect execution
        // timing in order to avoid execution delays to add up over time
        _job.lastExecution += _job.period;
        jobs[_job_id] = _job;

        // order next execution
        emit NextExecutionTimestamp(_job.lastExecution + _job.period, _job_id);
    }
}
