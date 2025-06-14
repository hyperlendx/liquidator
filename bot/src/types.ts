export interface Position {
    underlying: string;
    amount: string;
    decimals: string | number;
    price: string | number;
}

export interface PositionInfo {
    supply: Position[];
    borrow: Position[];
}

export interface Wallet {
    wallet_address: string;
}

export interface TokenHop {
    tokenIn: string;
    tokenOut: string;
    routerIndex: number;
    fee: number;
    amountIn: bigint;
    stable: boolean;
}

export interface AssetValue {
    underlying: string;
    value: number;
}

export interface SwapHop {
    tokenIn: string;
    tokenOut: string;
    routerIndex: number;
    fee: number;
    amountIn: number;
    stable: boolean;
}

export interface SwapPath {
    hop: SwapHop[];
    amountOut: string;
}

export interface SwapData {
    bestPath: SwapPath;
}

export interface SwapResponse {
    data: SwapData;
}

export interface TokenInfo {
    address: string;
    decimals: number;
}
export interface SwapAllocation {
    tokenIn: string;
    tokenOut: string;
    routerIndex: number;
    fee: number;
    amountIn: string | number;
    stable: boolean;
}

export interface SwapHop {
    allocations: SwapAllocation[];
}

export interface SwapPath {
    hop: SwapHop[];
    amountOut: string;
}

export interface TokenInfo {
    address: string;
    decimals: number;
}

export interface SwapTokenInfo {
    tokenIn: TokenInfo;
    tokenOut: TokenInfo;
    intermediate?: TokenInfo;
}

export interface SwapData {
    bestPath: SwapPath;
    tokenInfo: SwapTokenInfo;
}

export interface SwapResponse {
    data: SwapData;
}
