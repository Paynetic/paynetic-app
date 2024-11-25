// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {PayneticV1} from "../src/PayneticV1.sol";

contract PayneticV1Script is Script {
    PayneticV1 public paynetic;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        paynetic = new PayneticV1();

        vm.stopBroadcast();
    }
}
