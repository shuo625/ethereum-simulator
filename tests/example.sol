// SPDX-License-Identifier: MIT
// Tells the Solidity compiler to compile only from v0.8.13 to v0.9.0
pragma solidity ^0.8.13;

// This is just a simple example of a coin-like contract.
// It is not ERC20 compatible and cannot be expected to talk to other
// coin/token contracts.

contract MetaCoin {
    mapping(address => uint32) balances;

    constructor() {
        balances[tx.origin] = 10000;
    }

    function sendCoin(address receiver, uint32 amount)
        public
        returns (bool sufficient)
    {
        if (balances[tx.origin] < amount) return false;
        balances[tx.origin] -= amount;
        balances[receiver] += amount;

        return true;
    }

    function getBalance(address addr) public view returns (uint32) {
        return balances[addr];
    }
}
