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

### Deployment

#### Deploy the USDC contract
```bash
soroban contract deploy \
    --wasm token/target/wasm32-unknown-unknown/release/excellar_token_contract.wasm \
    --source-account S... \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022'
```
#### Initilize the test USDC contract

```bash
soroban contract invoke \
    --wasm token/target/wasm32-unknown-unknown/release/excellar_token_contract.wasm \
    --source-account S... \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id CAR2TYBD2SH24SVJUUA5G5RIUKYTV3BHUFP62UNCGX45RUOVXOHWCSBE \
    -- initialize --admin CAIAD7B6A5RCGJPUILM4CKAHOSBCSXLH2IO4EZLZG6IJ52WJJWDBXRE2 --decimal 6 --name 5553444320537461626c65 --symbol 55534443
```
#### Deploy the tokenizer contract
```bash
soroban contract deploy \
    --wasm tokenizer/target/wasm32-unknown-unknown/release/excellar_tokenizer_contract.wasm \
    --source-account S... \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022'
```

#### Initialize the test tokenizer contract
```bash
soroban contract invoke \
    --wasm tokenizer/target/wasm32-unknown-unknown/release/excellar_tokenizer_contract.wasm \
    --source-account S... \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id CAIAD7B6A5RCGJPUILM4CKAHOSBCSXLH2IO4EZLZG6IJ52WJJWDBXRE2 \
    -- initialize --token-wasm-hash 70c9fc851611f219d8beab55f4e06a9ff96f02749957fda390856a36e3770f33 --token-usdc CAR2TYBD2SH24SVJUUA5G5RIUKYTV3BHUFP62UNCGX45RUOVXOHWCSBE --admin GDOJ6OUGJYOQL2SQ52A2R33KOYHJMJ2DCLZZEYUXUKJBB3CSIO5ZKKQ5
```

### Testing the functionality

#### Invoke the USDC contract and mint tokens
```bash

soroban contract invoke \
    --wasm token/target/wasm32-unknown-unknown/release/excellar_token_contract.wasm \
    --source-account SCBDHL6YTFK4FUQIWRXPM2HZ6KAA7YECCOK6Y7RTYLTWRNJ2XDHHBH5R \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id CAR2TYBD2SH24SVJUUA5G5RIUKYTV3BHUFP62UNCGX45RUOVXOHWCSBE \
    -- mint --to {YOUR_XLM_PK} --amount 100
```

#### Deposit into the tokenizer contract
```bash
soroban contract invoke \
    --wasm tokenizer/target/wasm32-unknown-unknown/release/excellar_tokenizer_contract.wasm \
    --source-account {YOUR_XLM_SK}  \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id CAIAD7B6A5RCGJPUILM4CKAHOSBCSXLH2IO4EZLZG6IJ52WJJWDBXRE2 \
    -- deposit --to {YOUR_XLM_PK} --usdc-deposit 20
```

#### Withdraw own deposit from tokenizer contract
```bash
soroban contract invoke \
    --wasm tokenizer/target/wasm32-unknown-unknown/release/excellar_tokenizer_contract.wasm \
    --source-account {YOUR_XLM_SK}  \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id CAIAD7B6A5RCGJPUILM4CKAHOSBCSXLH2IO4EZLZG6IJ52WJJWDBXRE2 \
    -- withdraw --to {YOUR_XLM_PK} --share-amount 20
```
##### Withdraw all as admin

```bash
soroban contract invoke \
    --wasm tokenizer/target/wasm32-unknown-unknown/release/excellar_tokenizer_contract.wasm \
    --source-account SAVQKTSXS3T2VNXQRESDPWEAYT5HCSA6GRXPCGUF6HZDM2EOLGYDHFY6 \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --id CAIAD7B6A5RCGJPUILM4CKAHOSBCSXLH2IO4EZLZG6IJ52WJJWDBXRE2 \
    -- withdraw_admin --to GDOJ6OUGJYOQL2SQ52A2R33KOYHJMJ2DCLZZEYUXUKJBB3CSIO5ZKKQ5 --usdc-amount 20
```