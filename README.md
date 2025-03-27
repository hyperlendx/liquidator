# Aave V3 Liquidator

This is an [Artemis](https://github.com/paradigmxyz/artemis) bot that liquidates aave v3 positions on base chain

# Getting Started

1. Install Rust [ref](https://doc.rust-lang.org/book/ch01-01-installation.html)
    - Install deps: `sudo apt install -y build-essential libssl-dev` (Linux) or `xcode-select --install` (Mac)

3. Install Foundry [ref](https://book.getfoundry.sh/getting-started/installation)

3. Deploy liquidator contract
```bash
cd crates/liquidator-contract
forge install
forge create ./src/Liquidator.sol:Liquidator --private-key <xyz> --rpc-url <xyz> --broadcast
```

```
Deployer: 0x1B7a7d51eE86e1d9776986AEFD2675312CF0C9Da
Deployed to: 0x01141a2b3fd032Fb233043dE1225A64eC74e1cE0
Transaction hash: 0xacc52bb30639f5529e1491d7879847a36e5a6ccfe938cc818c08ffc34fce40c8
```

for mainnet verification:
```bash
forge create ./src/Liquidator.sol:Liquidator --private-key <xyz> --rpc-url <xyz> --broadcast --verify --verifier sourcify --verifier-url https://sourcify.parsec.finance/verify
```

4. Build Rust Application

a. cd to root of project

b. (if not cross compiling) Compile binary for host arch: 
```bash
cargo build --release
```

c. (if cross compiling) Compile binary on Mac for Linux: 
Cross is a tool developed by Rust devs to simplify cross compilation (compiling on one machine architecture for another architecture, e.g.: compile binary on mac for running on a linux machine). Make sure to install from github repo not latest release in cargo (it's quite outdated). [Github link](https://github.com/cross-rs/cross)

If you are using an ARM based Mac (e.g.: M1), you need to set DOCKER_DEFAULT_PLATFORM env to `linux/amd64`

```bash
cargo install cross --git https://github.com/cross-rs/cross
DOCKER_DEFAULT_PLATFORM=linux/amd64 cross build --target x86_64-unknown-linux-gnu --release
```

d. Copy (scp) binary to liquidator host

6. Run liquidator
```bash
./target/release/aave-v3-liquidator \
    --archive-rpc <xyz> \
    --write-rpc <xyz> \
    --private-key <xyz> \
    --bid-percentage 100 \
    --deployment hyperlend \
    --liquidator-address <xyz> \
    --chain-id <xyz>
```
```


# Re-generate Contract Bindings

## Liquidator
```bash
forge bind --bindings-path ./crates/bindings-liquidator --root ./crates/liquidator-contract --crate-name bindings-liquidator --overwrite
```

## Aave
```bash
forge bind --bindings-path ./crates/bindings-aave --root {path to aave-v3-core} --crate-name bindings-aave --overwrite
```