// SPDX-License-Identifier: MIT

///
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

/**
 * @title RecurringTransactions (..... MANAGOOOOOR)
 * @author malteish
 * @notice This contract allows for the creation of jobs that are executed with a fixed periodicity.
 * The first example implementation is a contract that sends a fixed amount of tokens to a recipient every period.
 * With very little modification, this contract could be used to do all sorts of naughty things. Be careful.
 * @dev In order to automatically execute the jobs, the contract needs to be triggered by an external entity.
 * This entity is expected to call the executeJob function at the right time. We leverage an Internet Computer
 * Canister for this purpose. The entity, called "trigger" in this contract, is expected to listen to the
 * NextExecutionTimestamp event, and call the executeJob function when THE TIME HATH COME1!1!
 */
contract RecurringTransactions {
    job[] public jobs;

    address payable public trigger;

    /**
     *
     * @param _trigger The address of the trigger contract that will call the executeJob function. Can be 0x0, which means it will be initialized later.
     * @dev Leaving this uninitialized is not safe, as it allows anyone to do the initializations later. We accept it as part of the MVP for now.
     * IF YOU ARE READING THIS, YOU ARE THE TRIGGER
     */
    constructor(address _trigger) {
        trigger = payable(_trigger);
    }

    /**
     *
     * @param date when to trigger the next execution of this job
     * @param job_id which job id to trigger
     * @dev This event is emitted whenever a new job is created, and whenever a job is executed. The external trigger
     * entity is expected to listen to this event and act accordingly.
     */
    event NextExecutionTimestamp(uint date, uint indexed job_id);

    /**
     * Init trigger if it has not been set yet
     * @param _trigger address of the trigger
     */
    function setTrigger(address _trigger) public {
        require(trigger == address(0), "Trigger already set");
        trigger = payable(_trigger);
    }

    /**
     * Internal function to transfer tokens
     * @param _token address of the ERC20 token contract
     * @param _sender one happy address
     * @param _recipient hope they have enough
     * @param _amount some, at least
     */
    function transferToken(
        address _token,
        address _sender,
        address _recipient,
        uint256 _amount
    ) public {
        IERC20 token = IERC20(_token);
        SafeERC20.safeTransferFrom(token, _sender, _recipient, _amount);
    }

    /**
     * Create as in before the job is not there, afterwards it has been executed once and will be executed
     * again in period seconds. And then again. And again. And again. And again. And again. And again. And again.
     * @param period time interval between to executions of the same job, in seconds
     * @param amount how many tokens to send to the recipient, in token bits (think wei for ETH)
     * @param recipient the lucky address getting the tokens
     * @param token the token's contract address
     * @dev only the sender can create a job. Safer this way, you know. I know you know. I know you know I know you know. Love you.
     */
    function createJob(
        uint256 period,
        uint256 amount,
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

    /**
     * Cancel a job. Can be done by the creator of the job.
     * @param _job_id the id of the job to remove once and for all. Never will there be known such a job again. EVER. Not kidding.
     */
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

    /**
     * Callable by the trigger to execute a job. Why only by the trigger? Because we said so. That's why.
     * @param _job_id the id of the job to execute
     */
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
