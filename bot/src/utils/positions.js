const axios = require("axios")
const { ethers } = require("ethers");

function pairLargestSupplyWithLargestBorrow(supplyBalances, borrowBalances) {
    // Filter out zero-value entries
    const sortedSupplies = supplyBalances
        .filter(e => e.value > 0)
        .sort((a, b) => b.value - a.value);

    const sortedBorrows = borrowBalances
        .filter(e => e.value > 0)
        .sort((a, b) => b.value - a.value);

    // Zip pairs by index
    const length = Math.min(sortedSupplies.length, sortedBorrows.length);
    const result = [];

    for (let i = 0; i < length; i++) {
        result.push([sortedSupplies[i], sortedBorrows[i]]);
    }

    return result;
}

async function getDetailedPosition(user) {
    const abi = [
        "function getAllSuppliedBalancesWithPrices(address pool, address user) view returns (tuple(address underlying, uint256 amount, uint256 price, uint256 decimals)[])",
        "function getAllBorrowedBalancesWithPrices(address pool, address user) view returns (tuple(address underlying, uint256 amount, uint256 price, uint256 decimals)[])"
    ];

    const provider = new ethers.JsonRpcProvider(process.env.RPC);
    const contract = new ethers.Contract(process.env.BALANCES_READER, abi, provider);

    try {
        const resultSupply = await contract.getAllSuppliedBalancesWithPrices(process.env.POOL, user);
        const resultBorrow = await contract.getAllBorrowedBalancesWithPrices(process.env.POOL, user);

        return {
            supply: resultSupply.map(entry => ({
                underlying: entry.underlying,
                amount: entry.amount.toString(),
                price: entry.price.toString(),
                decimals: entry.decimals.toString(),
            })),
            borrow: resultBorrow.map(entry => ({
                underlying: entry.underlying,
                amount: entry.amount.toString(),
                price: entry.price.toString(),
                decimals: entry.decimals.toString(),
            })),
        }
    } catch (err) {
        console.error("Failed to fetch data:", err);
        throw err;
    }
}

async function getLiquidatableWallets(){
    const data = (await axios.get(`https://hyperlend-api.blockanalitica.com/wallets/bad-debt-wallets/?network=hyper&order=-total_supply_usd&p=1&p_size=15`)).data
    return data.results;

    // const data = {
    //     results: [
    //         {
    //             "wallet_address": "0x0af3318c4060eac02d50e140de2fb0e492b59ecb",
    //             "total_supply": "2352.37022441269234673",
    //             "total_supply_change": "30.697184056199319558",
    //             "total_borrow": "948.8563641597621272",
    //             "total_borrow_change": "83.969150420123719364",
    //             "net": "1403.51386025300321491",
    //             "health_rate": "0.991665",
    //             "health_rate_change": "-0.082081",
    //             "emode_category": 1,
    //             "last_activity": "2025-05-14T16:26:04Z",
    //             "supplied_assets": [
    //                 {
    //                     "symbol": "UBTC",
    //                     "address": "0x9FDBdA0A5e284c32744D2f17Ee5c74B284993463"
    //                 }
    //             ],
    //             "borrowed_assets": [
    //                 {
    //                     "symbol": "WHYPE",
    //                     "address": "0x5555555555555555555555555555555555555555"
    //                 }
    //             ]
    //         },
    //         {
    //             "wallet_address": "0x205166e9cd418e207ab9cde137c2c5d933f29d7e",
    //             "total_supply": "74.856871372968637474",
    //             "total_supply_change": "-0.01858664335893096",
    //             "total_borrow": "54.29953040465301826",
    //             "total_borrow_change": "3.46221968137837370",
    //             "net": "20.55734132863356384",
    //             "health_rate": "0.972671",
    //             "health_rate_change": "-0.087674",
    //             "emode_category": 0,
    //             "last_activity": "2025-05-14T17:21:20Z",
    //             "supplied_assets": [
    //                 {
    //                     "symbol": "WHYPE",
    //                     "address": "0x5555555555555555555555555555555555555555"
    //                 },
    //                 {
    //                     "symbol": "USDe",
    //                     "address": "0x5d3a1ff2b6bab83b63cd9ad0787074081a52ef34"
    //                 },
    //                 {
    //                     "symbol": "UBTC",
    //                     "address": "0x9FDBdA0A5e284c32744D2f17Ee5c74B284993463"
    //                 },
    //                 {
    //                     "symbol": "USDT0",
    //                     "address": "0xB8CE59FC3717ada4C02eaDF9682A9e934F625ebb"
    //                 },
    //                 {
    //                     "symbol": "UETH",
    //                     "address": "0xBe6727B535545C67d5cAa73dEa54865B92CF7907"
    //                 }
    //             ],
    //             "borrowed_assets": [
    //                 {
    //                     "symbol": "wstHYPE",
    //                     "address": "0x94e8396e0869c9F2200760aF0621aFd240E1CF38"
    //                 },
    //                 {
    //                     "symbol": "UETH",
    //                     "address": "0xBe6727B535545C67d5cAa73dEa54865B92CF7907"
    //                 }
    //             ]
    //         }
    //     ]
    // }

    // return data.results;
}

module.exports = {
    pairLargestSupplyWithLargestBorrow: pairLargestSupplyWithLargestBorrow,
    getDetailedPosition: getDetailedPosition,
    getLiquidatableWallets: getLiquidatableWallets
}