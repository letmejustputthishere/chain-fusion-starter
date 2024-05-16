// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

contract Coprocessor {
    uint job_id = 0;
    address public coprocessor;

    constructor(address _coprocessor) {
        coprocessor = _coprocessor;
    }

    mapping(uint => string) public jobs;

    event NewJob(uint indexed job_id);

    // Function to create a new job
    function newJob() public payable {
        // Require at least 0.01 ETH to be sent with the call
        require(msg.value >= 0.01 ether, "Minimum 0.01 ETH not met");

        // Forward the ETH received to the coprocessor address
        // To pay for the submission of the job result back to the EVM 
        // contract.
        (bool success, ) = coprocessor.call{value: msg.value}("");
        require(success, "Failed to send Ether");

        // Emit the new job event
        emit NewJob(job_id);

        // Increment job counter
        job_id++;
    }


    function getResult(uint _job_id) public view returns (string memory) {
        return jobs[_job_id];
    }

    function callback(string calldata _result, uint256 _job_id) public {
        require(
            msg.sender == coprocessor,
            "Only the coprocessor can call this function"
        );
        jobs[_job_id] = _result;
    }
}