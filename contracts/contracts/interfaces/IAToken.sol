// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IAToken {
    function scaledBalanceOf(address user) external view returns (uint256);
    function balanceOf(address user) external view returns (uint256);
}