import { ethers } from 'ethers';
import { TokenHop, SwapResponse } from '../types';

export function prepareHops(apiResponse: SwapResponse['data']): TokenHop[][] {
    const hops: TokenHop[][] = [];

    const decimals: Record<string, number> = {};
    decimals[ethers.getAddress(apiResponse.tokenInfo.tokenIn.address)] =
        apiResponse.tokenInfo.tokenIn.decimals;
    decimals[ethers.getAddress(apiResponse.tokenInfo.tokenOut.address)] =
        apiResponse.tokenInfo.tokenOut.decimals;

    if (apiResponse.tokenInfo.intermediate) {
        decimals[ethers.getAddress(apiResponse.tokenInfo.intermediate.address)] =
            apiResponse.tokenInfo.intermediate.decimals;
    }

    for (const hop of apiResponse.bestPath.hop) {
        const allocations: TokenHop[] = hop.allocations.map((alloc) => ({
            tokenIn: alloc.tokenIn,
            tokenOut: alloc.tokenOut,
            routerIndex: alloc.routerIndex,
            fee: alloc.fee,
            amountIn: BigInt(
                Math.floor(
                    Number(alloc.amountIn) *
                        Math.pow(10, decimals[ethers.getAddress(alloc.tokenIn)])
                )
            ),
            stable: alloc.stable
        }));

        hops.push(allocations);
    }

    return hops;
}
