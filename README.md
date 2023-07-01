# lendcraft

Stellar LendCraft is a decentralized lending platform built on the Stellar Network. 
It allows users to tokenize, lend and borrow money market assets.

## Getting Started
```bash
rustup target add wasm32-unknown-unknown
cargo install --locked --version 0.8.0 soroban-cli
```

## Test
```bash
cargo test
```
## Build and deploy
```bash
cargo build --target wasm32-unknown-unknown --release
soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/lendcraft.wasm \
    --source S... \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022'
```

### Invoke
```bash
soroban contract invoke \
    --wasm target/wasm32-unknown-unknown/release/lendcraft.wasm \
    --source S... \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --contract-id c... \
    --fn hello \
    --fn-arg world
```