main()

async function main(){
    const Contract = await ethers.getContractFactory("Liquidator");
    const contract = await Contract.deploy('0x00a89d7a5a02160f20150ebea7a2b5e4879a1a8b');
    console.log(contract.target)
}