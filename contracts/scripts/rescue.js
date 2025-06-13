main()

async function main(){
    const liq = await ethers.getContractAt("Liquidator", "0xE2f16B804E2dD8170186de80219528eF4C3D73E6");
    console.log(await liq.rescueTokens('0x5555555555555555555555555555555555555555', 0, true, '0x9C1915f821912f5BbcbBD07EF92CEe9cF28068d4'))
}