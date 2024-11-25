// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {PayneticV1} from "../src/PayneticV1.sol";

contract PayneticV1Test is Test {

    address user1 = 0x0000000000000000000000000000000000000001;

    function setUp() public {
        hoax(user1, 5 ether);
    }
}
