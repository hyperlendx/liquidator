// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IDebtToken {
    function approveDelegation(address delegatee, uint256 amount) external;
    function scaledBalanceOf(address user) external returns (uint256);
}