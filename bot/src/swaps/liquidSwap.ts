import axios from "axios";
import { SwapResponse } from "../types";

export interface QuoteResponse {
    amount: number;
    rawAmount: string;
    data: any;
}

export async function getQuoteLiquidSwap(tokenIn: string, tokenOut: string, amountIn: number): Promise<QuoteResponse> {
    const swap: SwapResponse = (
        await axios.get<SwapResponse>(
            `https://api.liqd.ag/route?tokenA=${tokenIn}&tokenB=${tokenOut}&amountIn=${amountIn}&multiHop=true`
        )
    ).data;

    const tokenDecimals = swap.data.tokenInfo.tokenOut.decimals
    const amountOutRaw: string = (
        Number(swap.data.bestPath.amountOut) *
        Math.pow(10, Number(tokenDecimals))
    ).toFixed(0);

    return {
        amount: Number(swap.data.bestPath.amountOut),
        rawAmount: amountOutRaw,
        data: swap.data
    };
}