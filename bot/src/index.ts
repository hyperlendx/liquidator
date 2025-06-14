import dotenv from 'dotenv';
import axios from 'axios';
import cron from 'node-cron';
import { ethers } from 'ethers';

import { prepareHops } from './utils/swap';
import {
    pairLargestSupplyWithLargestBorrow,
    getDetailedPosition,
    getLiquidatableWallets
} from './utils/positions';

dotenv.config();

const CLOSE_FACTOR: number = 0.5;
const MAX_AMOUNT: string =
    '115792089237316195423570985008687907853269984665640564039457584007913129639935';

import {
    Position,
    PositionInfo,
    Wallet,
    TokenHop,
    AssetValue,
    SwapResponse
} from './types';

run();

cron.schedule('* * * * *', async (): Promise<void> => {
    console.log(`Running liquidator bot`)
    await run();
});

async function run(): Promise<void> {
    const wallets: Wallet[] = await getLiquidatableWallets();

    for (const wallet of wallets) {
        const positions: PositionInfo = await getDetailedPosition(wallet.wallet_address);

        const supply: AssetValue[] = positions.supply.map((pos): AssetValue => ({
            underlying: pos.underlying,
            value:
                (Number(pos.amount) / Math.pow(10, Number(pos.decimals))) *
                (Number(pos.price) / Math.pow(10, 8))
        }));

        const borrow: AssetValue[] = positions.borrow.map((pos): AssetValue => ({
            underlying: pos.underlying,
            value:
                (Number(pos.amount) / Math.pow(10, Number(pos.decimals))) *
                (Number(pos.price) / Math.pow(10, 8))
        }));

        console.log(`Found liquidation for ${wallet.wallet_address}`);

        const [pair]: [AssetValue, AssetValue][] = pairLargestSupplyWithLargestBorrow(supply, borrow);
        await prepareAndSend(wallet.wallet_address, pair, positions);
    }
}

async function prepareAndSend(
    user: string,
    pair: [AssetValue, AssetValue],
    positions: PositionInfo
): Promise<void> {
    const [supply, borrow] = pair;

    const pos: Position | undefined = positions.supply.find(
        (e) => e.underlying === supply.underlying
    );
    const posBorrow: Position | undefined = positions.borrow.find(
        (e) => e.underlying === borrow.underlying
    );

    if (!pos || !posBorrow) {
        console.error('Position not found');
        return;
    }

    const collateralAmount: string = (
        (Number(pos.amount) / Math.pow(10, Number(pos.decimals))) * CLOSE_FACTOR
    ).toFixed(Number(pos.decimals));

    const debtAmount: string = (
        Number(posBorrow.amount) / Math.pow(10, Number(posBorrow.decimals))
    ).toFixed(Number(posBorrow.decimals));

    console.log(`- collateral: ${collateralAmount} ${supply.underlying}`);
    console.log(`- debt: ${Number(debtAmount) * CLOSE_FACTOR} ${borrow.underlying}`);

    const swap: SwapResponse = (
        await axios.get<SwapResponse>(
            `https://api.liqd.ag/route?tokenA=${supply.underlying}&tokenB=${borrow.underlying}&amountIn=${collateralAmount}&multiHop=true`
        )
    ).data;

    const amountOutRaw: string = (
        Number(swap.data.bestPath.amountOut) *
        Math.pow(10, Number(posBorrow.decimals))
    ).toFixed(0);

    const debtToSeize: string =
        Number(swap.data.bestPath.amountOut) > Number(debtAmount) * CLOSE_FACTOR
            ? MAX_AMOUNT
            : amountOutRaw;

    console.log(`- seizing ${debtToSeize === MAX_AMOUNT ? 'MAX' : debtToSeize} of debt`);

    const hops: TokenHop[][] = prepareHops(swap.data);

    const tokens: string[] = [swap.data.bestPath.hop[0].tokenIn];
    for (const hop of swap.data.bestPath.hop || []) {
        tokens.push(hop.tokenOut);
    }

    await sendTx(
        user,
        supply.underlying,
        borrow.underlying,
        debtToSeize,
        hops,
        tokens,
        0
    );
}

async function sendTx(
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
