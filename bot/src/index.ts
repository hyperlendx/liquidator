import dotenv from 'dotenv';
import axios from 'axios';
import cron from 'node-cron';
import { ethers } from 'ethers';

import { prepareHops } from './utils/swap';
import {
    pairLargestSupplyWithLargestBorrow,
    getDetailedPosition,
    getLiquidatableWallets,
    getLiquidatablePositions,
    UserPositions
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
import { sendTx } from './utils/tx';
import { getQuoteLiquidSwap, QuoteResponse } from './swaps/liquidSwap';

run();

cron.schedule('* * * * *', async (): Promise<void> => {
    console.log(`Running liquidator bot`)
    await run();
});

async function run(): Promise<void> {
    //find all wallets that can be liquidated
    const wallets: Wallet[] = await getLiquidatableWallets();
    //find their positions
    const userPositions: UserPositions[] = await getLiquidatablePositions(wallets);

    //for each liquidatable user
    for (let pos of userPositions){
        await prepareAndSend(pos.address, pos.pair, pos.positions);
    }
}

async function prepareAndSend(
    user: string,
    pair: [AssetValue, AssetValue],
    positions: PositionInfo
): Promise<void> {
    const [supply, borrow] = pair;

    const posCollateral: Position | undefined = positions.supply.find(
        (e) => e.underlying === supply.underlying
    );
    const posBorrow: Position | undefined = positions.borrow.find(
        (e) => e.underlying === borrow.underlying
    );

    if (!posCollateral || !posBorrow) {
        console.error('Position not found');
        return;
    }

    const collateralAmount: string = (
        (Number(posCollateral.amount) / Math.pow(10, Number(posCollateral.decimals))) * CLOSE_FACTOR
    ).toFixed(Number(posCollateral.decimals));

    const debtAmount: string = (
        Number(posBorrow.amount) / Math.pow(10, Number(posBorrow.decimals))
    ).toFixed(Number(posBorrow.decimals));

    console.log(`- user: ${user}`)
    console.log(`- collateral: ${collateralAmount} ${supply.underlying}`);
    console.log(`- liquidatable debt: ${Number(debtAmount) * CLOSE_FACTOR} ${borrow.underlying}`);


    //get quotes for atomic liquidation
    // const liquidSwapQuote: QuoteResponse = getQuoteLiquidSwap(supply.underlying, borrow.underlying, Number(collateralAmount));

    //TODO move quote selection to another file, so we only get best quote result here
    const bestSwap: QuoteResponse = await getQuoteLiquidSwap(supply.underlying, borrow.underlying, Number(collateralAmount));

    const debtToSeize: string =
        bestSwap.amount > Number(debtAmount) * CLOSE_FACTOR
            ? MAX_AMOUNT
            : bestSwap.rawAmount;

    console.log(`- seizing ${debtToSeize === MAX_AMOUNT ? 'MAX' : debtToSeize} of debt`);

    const hops: TokenHop[][] = prepareHops(bestSwap.data);

    const tokens: string[] = [bestSwap.data.bestPath.hop[0].tokenIn];
    for (const hop of bestSwap.data.bestPath.hop || []) {
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
