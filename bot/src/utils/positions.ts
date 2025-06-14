// positions.ts
import axios from 'axios';
import { ethers } from 'ethers';
import { AssetValue, Position, PositionInfo, Wallet } from '../types';

export interface UserPositions {
    address: string;
    pair: [AssetValue, AssetValue],
    positions: PositionInfo
}

export async function getLiquidatablePositions(wallets: Wallet[]): Promise<UserPositions[]> {
    const returnPositions: UserPositions[] = [];

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

        //create pair of largest borrow-largest collateral to maximize profit when liquidating
        const [pair]: [AssetValue, AssetValue][] = pairLargestSupplyWithLargestBorrow(supply, borrow);

        returnPositions.push({
            address: wallet.wallet_address,
            pair: pair,
            positions: positions
        })
    }

    return returnPositions;
}

export function pairLargestSupplyWithLargestBorrow(
    supplyBalances: AssetValue[],
    borrowBalances: AssetValue[]
): [AssetValue, AssetValue][] {
    const sortedSupplies = supplyBalances
        .filter(e => e.value > 0)
        .sort((a, b) => b.value - a.value);

    const sortedBorrows = borrowBalances
        .filter(e => e.value > 0)
        .sort((a, b) => b.value - a.value);

    const length = Math.min(sortedSupplies.length, sortedBorrows.length);
    const result: [AssetValue, AssetValue][] = [];

    for (let i = 0; i < length; i++) {
        result.push([sortedSupplies[i], sortedBorrows[i]]);
    }

    return result;
}

export async function getDetailedPosition(user: string): Promise<PositionInfo> {
    const abi = [
        'function getAllSuppliedBalancesWithPrices(address pool, address user) view returns (tuple(address underlying, uint256 amount, uint256 price, uint256 decimals)[])',
        'function getAllBorrowedBalancesWithPrices(address pool, address user) view returns (tuple(address underlying, uint256 amount, uint256 price, uint256 decimals)[])'
    ];

    const provider = new ethers.JsonRpcProvider(process.env.RPC);
    const contract = new ethers.Contract(process.env.BALANCES_READER as string, abi, provider);

    try {
        const resultSupply = await contract.getAllSuppliedBalancesWithPrices(process.env.POOL, user);
        const resultBorrow = await contract.getAllBorrowedBalancesWithPrices(process.env.POOL, user);

        return {
            supply: resultSupply.map((entry: any): Position => ({
                underlying: entry.underlying,
                amount: entry.amount.toString(),
                price: entry.price.toString(),
                decimals: entry.decimals.toString()
            })),
            borrow: resultBorrow.map((entry: any): Position => ({
                underlying: entry.underlying,
                amount: entry.amount.toString(),
                price: entry.price.toString(),
                decimals: entry.decimals.toString()
            }))
        };
    } catch (err) {
        console.error('Failed to fetch data:', err);
        throw err;
    }
}

export async function getLiquidatableWallets(): Promise<Wallet[]> {
    const response = await axios.get(
        'https://hyperlend-api.blockanalitica.com/wallets/bad-debt-wallets/?network=hyper&order=-total_supply_usd&p=1&p_size=15'
    );
    return response.data.results;
}
