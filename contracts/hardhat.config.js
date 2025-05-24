require("dotenv").config();
require("@nomicfoundation/hardhat-toolbox");

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
    solidity: "0.8.20",
        networks: {
        hyperEvm: {
            accounts: [process.env.PRIVATE_KEY_MAINNET],
            chainId: 999,
            url: 'https://rpc.hyperliquid.xyz/evm',
        }
    },
};
