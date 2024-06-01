// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

import "../../../lib/forge-std/src/Script.sol";
import "../Coprocessor.sol";

contract Start is Script {
    uint256 accountPrivateKey;
    address accountAddress;

    function setUp() public {
        // accountPrivateKey = vm.envUint("ANVIL_PRIVATE_KEY_0");
        // accountAddress = vm.addr(accountPrivateKey);
    }

    function run() public {
        // local testnet
        Coprocessor coprocessor = Coprocessor(
            0x5FbDB2315678afecb367f032d93F642f64180aa3
        );

        // load contract bytecode
        bytes memory bytecode = type(Coprocessor).creationCode;
        console.log("Bytecode length: %d", bytecode.length);

        console.log("Calling newJob...");
        vm.startBroadcast();
        // payable(address(0xDFcEB49eD21aE199b33A76B726E2bea7A72127B0)).transfer(
        //     300 ether
        // );
        coprocessor.newJob{value: 0.1 ether}();
        vm.stopBroadcast();
    }
}
