#![allow(unused)]
use soroban_sdk::{xdr::ToXdr, Address, Bytes, BytesN, Env};

soroban_sdk::contractimport!(
    file = "../token/target/wasm32-unknown-unknown/release/excellar_token_contract.wasm"
);

pub fn create_contract(e: &Env, token_wasm_hash: &BytesN<32>, token_usdc: &Address) -> Address {
    let mut salt = Bytes::new(e);
    salt.append(&token_usdc.to_xdr(e));
    let salt = e.crypto().sha256(&salt);
    e.deployer()
        .with_current_contract(&salt)
        .deploy(token_wasm_hash)
}
