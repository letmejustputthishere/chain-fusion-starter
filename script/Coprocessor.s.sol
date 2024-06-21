// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "../contracts/Coprocessor.sol";

contract MyScript is Script {
    function run(address chain_fusion_canister_address) external {
        // the private key of the deployer is the first private key printed by running anvil
        uint256 deployerPrivateKey = 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80;
        // we use that key to broadcast all following transactions
        vm.startBroadcast(deployerPrivateKey);

        // this creates the contract. it will have the same address every time if we use a 
        // new instance of anvil for every deployment.

        Coprocessor coprocessor = new Coprocessor(chain_fusion_canister_address);

       // we create 3 jobs
        for (uint256 index = 0; index < 3; index++) {
            coprocessor.newJob{value: 0.1 ether}();
        }

        vm.stopBroadcast();
    }
}
