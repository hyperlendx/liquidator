require("dotenv").config()
const axios = require("axios")
const cron = require('node-cron');
const { ethers } = require("ethers");


const CLOSE_FACTOR = 0.5 //close 50% of the position
const MAX_AMOUNT = '115792089237316195423570985008687907853269984665640564039457584007913129639935'

const { prepareHops } = require("./utils/swap")
const {
    pairLargestSupplyWithLargestBorrow,
    getDetailedPosition,
    getLiquidatableWallets
} = require("./utils/positions")

run()

cron.schedule('* * * * *', async () => {
    run()
});

async function run(){
    const wallets = await getLiquidatableWallets()

    for (let wallet of wallets){
        //get detailed users position
        const positions = await getDetailedPosition(wallet.wallet_address)

        //prepare position values
        let supply = []
        for (let pos of positions.supply){
            let value = (Number(pos.amount) / Math.pow(10, Number(pos.decimals))) * (Number(pos.price) / Math.pow(10, 8))
            supply.push({ 
                underlying: pos.underlying,
                value: value
            })
        }

        let borrow = []
        for (let pos of positions.borrow){
            let value = (Number(pos.amount) / Math.pow(10, Number(pos.decimals))) * (Number(pos.price) / Math.pow(10, 8))
            borrow.push({ 
                underlying: pos.underlying,
                value: value
            })
        }

        console.log(`Found liquidation for ${wallet.wallet_address}`)

        //prepare pairs by size
        const [pair] = pairLargestSupplyWithLargestBorrow(supply, borrow)
        await prepareAndSend(wallet.wallet_address, pair, positions)
    }
}

async function prepareAndSend(user, pair, positions){
    const supply = pair[0]
    const borrow = pair[1]
    
    const pos = positions.supply.find(e => e.underlying == supply.underlying)
    const posBorrow = positions.borrow.find(e => e.underlying == borrow.underlying)

    const collateralAmount = parseFloat((Number(pos.amount) / Math.pow(10, Number(pos.decimals))) * CLOSE_FACTOR).toFixed(Number(pos.decimals))
    const debtAmount = parseFloat((Number(posBorrow.amount) / Math.pow(10, Number(posBorrow.decimals)))).toFixed(Number(posBorrow.decimals))

    console.log(`- collateral: ${collateralAmount} ${supply.underlying}`)
    console.log(`- debt: ${debtAmount} ${borrow.underlying}`)

    //get swap route
    let swap = (await axios.get(`https://api.liqd.ag/route?tokenA=${supply.underlying}&tokenB=${borrow.underlying}&amountIn=${collateralAmount}&multiHop=true`)).data

    const amountOutRaw = parseFloat(Number(swap.data.bestPath.amountOut) * Math.pow(10, Number(posBorrow.decimals))).toFixed(0)
    const debtToSeize = Number(swap.data.bestPath.amountOut) > debtAmount ? MAX_AMOUNT : amountOutRaw;
    
    console.log(`- seizing ${debtToSeize == MAX_AMOUNT ? "MAX" : debtToSeize} of debt`)

    const hops = prepareHops(swap.data)

    let tokens = [swap?.data.bestPath?.hop[0].tokenIn];
    for (let i = 0; i < swap?.data.bestPath?.hop?.length; i++) {
        tokens.push(swap?.data.bestPath?.hop[i]?.tokenOut);
    }
    
    await sendTx(user, supply.underlying, borrow.underlying, debtToSeize, hops, tokens, 0)
}

async function sendTx(user, collateral, debt, debtAmount, hops, tokens, minAmountOut){
    const provider = new ethers.JsonRpcProvider(process.env.SEND_RPC);
    const signer = new ethers.Wallet(process.env.PRIVATE_KEY, provider);

    const abi = [
        `function liquidate(address _user, address _collateral, address _debt, uint256 _debtAmount, tuple(address tokenIn, address tokenOut, uint8 routerIndex, uint24 fee, uint256 amountIn, bool stable)[][] _hops, address[] _tokens, uint256 _minAmountOut)`,
        `function rescueTokens(address _token, uint256 _amount, bool _max, address _to)`
    ];
    const contract = new ethers.Contract(process.env.LIQUIDATOR, abi, signer);
    
    const tx = await contract.liquidate(
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

    //send profit to another wallet
    let profit = await contract.rescueTokens(collateral, 0, true, process.env.PROFIT_RECEIVER)
    console.log(`removing profit: ${profit.hash}`)
    await profit.wait()
}





