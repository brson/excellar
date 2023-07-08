# excellar

Stellar excellar is a decentralized lending platform built on the Stellar Network. 
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
    --wasm target/wasm32-unknown-unknown/release/excellar.wasm \
    --source S... \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022'
```

### Deploy

```bash
soroban contract deploy \
    --wasm token/target/wasm32-unknown-unknown/release/excellar_token_contract.wasm \
    --source-account S... \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022'
    
soroban contract invoke \
    --wasm token/target/wasm32-unknown-unknown/release/excellar_token_contract.wasm \
    --source-account S... \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id CAR2TYBD2SH24SVJUUA5G5RIUKYTV3BHUFP62UNCGX45RUOVXOHWCSBE \
    -- initialize --admin CBGKDJ7R6UPYVCCVZJHWW6Z2PGHGWUNOJKDQ3MUYQIULPBGGMLFAUBEN --decimal 6 --name 5553444320537461626c65 --symbol 55534443

soroban contract deploy \
    --wasm tokenizer/target/wasm32-unknown-unknown/release/excellar_tokenizer_contract.wasm \
    --source-account S... \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022'
    
soroban contract invoke \
    --wasm tokenizer/target/wasm32-unknown-unknown/release/excellar_tokenizer_contract.wasm \
    --source-account S... \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id CBGKDJ7R6UPYVCCVZJHWW6Z2PGHGWUNOJKDQ3MUYQIULPBGGMLFAUBEN \
    -- initialize --token-wasm-hash 70c9fc851611f219d8beab55f4e06a9ff96f02749957fda390856a36e3770f33 --token-usdc CAR2TYBD2SH24SVJUUA5G5RIUKYTV3BHUFP62UNCGX45RUOVXOHWCSBE
```



### Invoke

#### Invoke the USDC contract and mint tokens
```bash

soroban contract invoke \
    --wasm token/target/wasm32-unknown-unknown/release/excellar_token_contract.wasm \
    --source-account S... \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id CAR2TYBD2SH24SVJUUA5G5RIUKYTV3BHUFP62UNCGX45RUOVXOHWCSBE \
    -- mint --to GBGXBIEMYC7F2OVWVXKNJVYXSRUS4BXF57L5IZWHMDJIPTFPP5Z7TNIP --amount 100
```
#### Invoke the tokenizer contract

```bash
soroban contract invoke \
    --wasm tokenizer/target/wasm32-unknown-unknown/release/excellar_tokenizer_contract.wasm \
    --source-account S... \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id CBGKDJ7R6UPYVCCVZJHWW6Z2PGHGWUNOJKDQ3MUYQIULPBGGMLFAUBEN \
    -- deposit --to GBGXBIEMYC7F2OVWVXKNJVYXSRUS4BXF57L5IZWHMDJIPTFPP5Z7TNIP --usdc-deposit 20


soroban contract invoke \
    --wasm tokenizer/target/wasm32-unknown-unknown/release/excellar_tokenizer_contract.wasm \
    --source-account S... \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id CBGKDJ7R6UPYVCCVZJHWW6Z2PGHGWUNOJKDQ3MUYQIULPBGGMLFAUBEN \
    -- withdraw --to GBGXBIEMYC7F2OVWVXKNJVYXSRUS4BXF57L5IZWHMDJIPTFPP5Z7TNIP --share-amount 20
```