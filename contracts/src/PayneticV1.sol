// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/console2.sol";

contract PayneticV1 {
    bool private _entered;

    modifier nonReentrant() {
        require(!_entered, "re-entrant blocked");
        _entered = true;
        _;
        _entered = false;
    }
    string public version;

    constructor() {
        version = "1";
    }

}
