// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { IERC20Metadata } from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import { ReentrancyGuard } from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

import { ILiquidSwap } from "./interfaces/ILiquidSwap.sol";
import { IPool } from "./interfaces/IPool.sol";
import { IWrappedHype } from "./interfaces/IWrappedHype.sol";

contract Liquidator is Ownable, ReentrancyGuard {
    using SafeERC20 for IERC20;

    IPool public pool;
    ILiquidSwap public liquidSwapRouter = ILiquidSwap(0x744489Ee3d540777A66f2cf297479745e0852f7A);
    IWrappedHype public WHYPE = IWrappedHype(0x5555555555555555555555555555555555555555);

    constructor(address _pool) Ownable(msg.sender) {
        pool = IPool(_pool);
    }

    function liquidate(address _user, address _collateral, address _debt, uint256 _debtAmount, ILiquidSwap.Swap[][] memory _hops, address[] memory _tokens, uint256 _minAmountOut) external onlyOwner() {
        //find required debt amount
        if (_debtAmount == type(uint256).max){
            address dToken = (IPool(pool).getReserveData(_debt)).variableDebtTokenAddress;
            _debtAmount = IERC20(dToken).balanceOf(_user) / 2;
        }

        LiquidationParams memory liqParams = LiquidationParams({
            user: _user,
            collateral: _collateral,
            debtToCover: _debtAmount,
            hops: _hops,
            tokens: _tokens,
            minAmountOut: _minAmountOut
        });
        bytes memory params = abi.encode(liqParams); 
        pool.flashLoanSimple(address(this), _debt, _debtAmount, params, 0);
    }

    struct LiquidationParams {
        address user;
        address collateral;
        uint256 debtToCover;
        ILiquidSwap.Swap[][] hops;
        address[] tokens;
        uint256 minAmountOut;
    }

    event Balance(uint256 amount);

    function executeOperation(
        address debtAsset,
        uint256 amount,
        uint256 premium,
        address initiator,
        bytes calldata params
    )  external returns (bool) {
        require(msg.sender == address(pool), "msg.sender != pool");
        require(initiator == address(this), "initiator != address(this)");

        LiquidationParams memory liqParams = abi.decode(params, (LiquidationParams));

        //liquidate user
        IERC20(debtAsset).approve(address(pool), type(uint256).max);
        pool.liquidationCall(liqParams.collateral, debtAsset, liqParams.user, liqParams.debtToCover, false);

        //swap collateral to debtAsset
        uint256 balance = IERC20(liqParams.collateral).balanceOf(address(this));
        IERC20(liqParams.collateral).approve(address(liquidSwapRouter), balance);

        //check if we need to adjust amount
        uint256 inputAmountFromHops = 0;
        for (uint256 i = 0; i < liqParams.hops[0].length; i++) {
            inputAmountFromHops += liqParams.hops[0][i].amountIn;
        }
        if (inputAmountFromHops > balance) {
            liqParams.hops[0][0].amountIn -= (inputAmountFromHops - balance);
        }
        if (balance > inputAmountFromHops){
            liqParams.hops[0][0].amountIn += (inputAmountFromHops - balance);
        }

        //swap
        liquidSwapRouter.executeMultiHopSwap(liqParams.tokens, balance, liqParams.minAmountOut, liqParams.hops);

        if (address(this).balance > 0){
            WHYPE.deposit{value: address(this).balance}();
        }

        require(IERC20(debtAsset).balanceOf(address(this)) >= amount + premium, "insufficient output of the swap");

        return true;
    }

    function rescueTokens(address _token, uint256 _amount, bool _max, address _to) external onlyOwner(){
        if (_token == address(0)){
            if (_max) _amount = address(this).balance;
            (bool success, ) = payable(_to).call{value: _amount}("");
            require(success, "transfer failed");
        } else {
            if (_max) _amount = IERC20(_token).balanceOf(address(this));
            IERC20(_token).safeTransfer(_to, _amount);
        }
    }
}
