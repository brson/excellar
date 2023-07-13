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

## Contract Purpose
The following contract allows customers to deposit USDC collateral and get on chain representation in the form of XUSG of a real world bond. 
Additionally, admins can periodically update the asset price, cash reserves and fees, associated with managing the contract.

## Contract API 

- `soroban {...params} initialize --token_wasm_hash=<BytesN<32>> --token_usdc=<Address> --admin=<Address>`: Initializes the contract with the provided token_wasm_hash, token_usdc, and admin address.

- `soroban {...params} xusg_id`: Fetches and returns the ID/address of the XUSG token in the contract.

- `soroban {...params} etf_market_value`: Fetches and returns the value of the ETF value in the contract.

- `soroban {...params} set_etf_market_value --value=<i128>`: Sets the value of the ETF in the contract (admin only).

- `soroban {...params} cash_reserves`: Fetches and returns the amount of cash reserves in the contract.

- `soroban {...params} set_cash_reserves --amount=<i128>`: Sets the amount of cash reserves in the contract (admin only).

- `soroban {...params} fees`: Fetches and returns the fees in the contract.

- `soroban {...params} set_fees --amount=<i128>`: Sets the fees in the contract (admin only).

- `soroban {...params} deposit --to=<Address> --usdc_amount=<i128>`: Allows a user to deposit a specified amount into the contract and mints the equivalent XUSG tokens.

- `soroban {...params} withdraw --to=<Address> --xusg_amount=<i128>`: Allows a user to withdraw a specified amount of their XUSG from the contract and returns the equivalent USDC amount.

- `soroban {...params} withdraw_admin --to=<Address> --usdc_amount=<i128>`: Allows the admin to withdraw a specified amount of USDC from the contract. (admin only)

- `soroban {...params} balance --account=<Address>`: Fetches and returns the balance of XUSG tokens for a given account.

- `soroban {...params} price`: Calculates and returns the current price of the XUSG token.

- `soroban {...params} total`: Fetches and returns the total amount of XUSG tokens in the contract.

## Testing the functionality

#### Prerequisite
It's assumed that you already have [soroban-cli](https://soroban.stellar.org/docs/getting-started/setup#install-the-soroban-cli) installed

#### Load the depositing wallet with some collateral
We need to load the wallet with some collateral that we can use to deposit in the tokenizer contract.

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
We deposit the USDC collateral into the tokenizer contract, in order to get XUSG minted in return, by calling the `deposit` method.

```bash
soroban contract invoke \
  --wasm tokenizer/target/wasm32-unknown-unknown/release/excellar_tokenizer_contract.wasm \
  --source-account {YOUR_XLM_SK} \
  --rpc-url https://rpc-futurenet.stellar.org:443 \
  --network-passphrase 'Test SDF Future Network ; October 2022' \
  --id CADNYYFRDL3L4FSUINUK7OQTSCUUWHIWE6KSCTN3TWCIEKJC42OKJK7Y \
  -- deposit --to {YOUR_XLM_PK} --usdc-deposit 20
```

#### Withdraw own deposit from tokenizer contract
If the price of XUSG has gone up or if we want to lower our exposure, we can burn the XUSG we have and get back the USDC collateral, by calling the `withdraw` function as specify the amount of XUSG we want to withdraw.

```bash
soroban contract invoke \
  --wasm tokenizer/target/wasm32-unknown-unknown/release/excellar_tokenizer_contract.wasm \
  --source-account {YOUR_XLM_SK} \
  --rpc-url https://rpc-futurenet.stellar.org:443 \
  --network-passphrase 'Test SDF Future Network ; October 2022' \An
  --id CADNYYFRDL3L4FSUINUK7OQTSCUUWHIWE6KSCTN3TWCIEKJC42OKJK7Y \
  -- withdraw --to {YOUR_XLM_PK} --share-amount 20
```
##### Withdraw all as admin

Administrators can withdraw all the USDC collateral from the contract, in order to buy the real world asset.

```bash
soroban contract invoke \
  --wasm tokenizer/target/wasm32-unknown-unknown/release/excellar_tokenizer_contract.wasm \
  --source-account SAVQKTSXS3T2VNXQRESDPWEAYT5HCSA6GRXPCGUF6HZDM2EOLGYDHFY6 \
  --rpc-url https://rpc-futurenet.stellar.org:443 \
  --network-passphrase 'Test SDF Future Network ; October 2022' \
  --id CADNYYFRDL3L4FSUINUK7OQTSCUUWHIWE6KSCTN3TWCIEKJC42OKJK7Y \
  -- withdraw_admin --to GDOJ6OUGJYOQL2SQ52A2R33KOYHJMJ2DCLZZEYUXUKJBB3CSIO5ZKKQ5 --usdc-amount 20
```
## Price Calculation
**N.B.** The `etf_market_value`, `cash_reserves`, `fees` and `total_xusg` are all stored in the contract as `i128` values. The first two digits represent cents and tens of cents, whenever price calculation is involved.
The `calculate_xusg_price` function calculates the price of the XUSG token based on the following parameters:

1. `cash_reserves`: The total cash reserves on hand.
2. `fees`: The total amount of fees that have been charged for managing the fund.
3. `etf_market_value`: The market value of the ETF that is tracking the real world asset.
4. `total_xusg`: The total amount of XUSG tokens issued.

The price of the XUSG token is calculated using the following formula:
`(etf_market_value + cash_reserves - fees) / total_xusg`

The function first checks if the `total_xusg` is greater than 0. If not, it returns a default price of 1.

Then, it calculates the price as the sum of the `etf_market_value` and `cash_reserves` minus `fees`, divided by `total_xusg`. The result is the price of one XUSG token.

Let's assume the following current state:

- `etf_market_value` = 0 (no money has been invested yet)
- `cash_reserves` = 5000
- `fees` = 0
- `total_xusg` = 5000
So the `xusg_price` would be calculated as: `(etf_market_value + cash_reserves - fees) / total_xusg = (0 + 5000 - 0) / 5000 = 1`

Once we buy the ETF, then the calculation becomes: `(etf_market_value + cash_reserves - fees) / total_xusg = (5000 + 0 - 0) / 5000 = 1`

If the price of the ETF was to double then `xusg_price` becomes: `(etf_market_value + cash_reserves - fees) / total_xusg = (10000 + 0 - 0) / 5000 = 2`

You will notice that small movements in the price of the ETF will not affect the price of the bond on chain.