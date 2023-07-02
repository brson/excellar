#![cfg(test)]
extern crate std;

use crate::{token, LendCraftTokenizerClient};

use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, IntoVal, Symbol};

fn create_token_contract<'a>(e: &Env, admin: &Address) -> token::Client<'a> {
    token::Client::new(e, &e.register_stellar_asset_contract(admin.clone()))
}

fn create_tokenizer_contract<'a>(
    e: &Env,
    token_wasm_hash: &BytesN<32>,
    token_a: &Address,
) -> LendCraftTokenizerClient<'a> {
    let tokenizer =
        LendCraftTokenizerClient::new(e, &e.register_contract(None, crate::LendCraftTokenizer {}));
    tokenizer.initialize(token_wasm_hash, token_a);
    tokenizer
}

fn install_token_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../token/target/wasm32-unknown-unknown/release/lendcraft_token_contract.wasm"
    );
    e.install_contract_wasm(WASM)
}

#[test]
fn test_multi_user_deposit() {
    let e = Env::default();
    e.mock_all_auths();

    let admin1 = Address::random(&e);
    let token_usdc = create_token_contract(&e, &admin1);
    let user1 = Address::random(&e);
    let user2 = Address::random(&e);
    let tokenizer = create_tokenizer_contract(&e, &install_token_wasm(&e), &token_usdc.address);

    let token_share = token::Client::new(&e, &tokenizer.share_id());

    token_usdc.mint(&user1, &40);
    token_usdc.mint(&user2, &70);
    assert_eq!(token_usdc.balance(&user1), 40);
    assert_eq!(token_usdc.balance(&user2), 70);

    tokenizer.deposit(&user1, &10);
    assert_eq!(
        e.auths(),
        [
            (
                user1.clone(),
                tokenizer.address.clone(),
                Symbol::short("deposit"),
                (&user1, 10_i128).into_val(&e)
            ),
            (
                user1.clone(),
                token_usdc.address.clone(),
                Symbol::short("transfer"),
                (&user1, &tokenizer.address, 10_i128).into_val(&e)
            )
        ]
    );

    tokenizer.deposit(&user2, &20);
    assert_eq!(
        e.auths(),
        [
            (
                user2.clone(),
                tokenizer.address.clone(),
                Symbol::short("deposit"),
                (&user2, 20_i128).into_val(&e)
            ),
            (
                user2.clone(),
                token_usdc.address.clone(),
                Symbol::short("transfer"),
                (&user2, &tokenizer.address, 20_i128).into_val(&e)
            ),
        ]
    );

    assert_eq!(token_share.balance(&user1), 10);
    assert_eq!(token_share.balance(&tokenizer.address), 0);
    assert_eq!(token_usdc.balance(&user1), 30);
    assert_eq!(token_usdc.balance(&tokenizer.address), 30);

    assert_eq!(token_share.balance(&user2), 20);
    assert_eq!(token_share.balance(&tokenizer.address), 0);
    assert_eq!(token_usdc.balance(&user2), 50);
    assert_eq!(token_usdc.balance(&tokenizer.address), 30);

    tokenizer.withdraw(&user1, &7);
    assert_eq!(
        e.auths(),
        [
            (
                user1.clone(),
                tokenizer.address.clone(),
                Symbol::short("withdraw"),
                (&user1, 7_i128).into_val(&e)
            ),
            (
                user1.clone(),
                token_share.address.clone(),
                Symbol::short("transfer"),
                (&user1, &tokenizer.address, 7_i128).into_val(&e)
            )
        ]
    );

    assert_eq!(token_usdc.balance(&user1), 37);
    assert_eq!(token_share.balance(&user1), 3);
    assert_eq!(token_usdc.balance(&user2), 50);
    assert_eq!(token_share.balance(&user2), 20);
    assert_eq!(token_usdc.balance(&tokenizer.address), 23);
    assert_eq!(token_share.balance(&tokenizer.address), 0);
}
