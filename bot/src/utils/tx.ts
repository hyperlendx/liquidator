import { ethers } from "ethers";
import { TokenHop } from "../types";

export async function sendTx(
    user: string,
    collateral: string,
    debt: string,
    debtAmount: string,
    hops: TokenHop[][],
    tokens: string[],
    minAmountOut: number
): Promise<void> {
    const provider: ethers.JsonRpcProvider = new ethers.JsonRpcProvider(
        process.env.SEND_RPC
    );
    const signer: ethers.Wallet = new ethers.Wallet(
        String(process.env.PRIVATE_KEY),
        provider
    );

    const abi: string[] = [
        `function liquidate(address _user, address _collateral, address _debt, uint256 _debtAmount, tuple(address tokenIn, address tokenOut, uint8 routerIndex, uint24 fee, uint256 amountIn, bool stable)[][] _hops, address[] _tokens, uint256 _minAmountOut)`,
        `function rescueTokens(address _token, uint256 _amount, bool _max, address _to)`
    ];

    const contract: ethers.Contract = new ethers.Contract(
        String(process.env.LIQUIDATOR),
        abi,
        signer
    );

    const tx: ethers.ContractTransactionResponse = await contract.liquidate(
        user,
        collateral,
        debt,
        debtAmount,
        hops,
        tokens,
        minAmountOut
    );

    console.log('Tx sent:', tx.hash);
    await tx.wait();
    console.log('Tx confirmed!');

    const profitTx: ethers.ContractTransactionResponse = await contract.rescueTokens(
        debt,
        0,
        true,
        String(process.env.PROFIT_RECEIVER)
    );

    console.log(`Removing profit: ${profitTx.hash}`);
    await profitTx.wait();
}