// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

contract Box {
    uint public x;

    constructor() {
        x = 10;
      }

    function store(uint _x) public {
        x = _x;
    }

    function get() public view returns (uint) {}
}
