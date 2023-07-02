#![no_std]

mod error;
mod test;
mod token;

use crate::error::LendCraftError;
use soroban_sdk::{
    contractimpl, contractmeta, Address, Bytes, BytesN, ConversionError, Env, RawVal, TryFromVal,
};
use token::create_contract;

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum DataKey {
    TokenUSDC = 1,
    TokenShare = 2,
    TotalShares = 3,
    ReserveUSDC = 4,
}

impl TryFromVal<Env, DataKey> for RawVal {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &DataKey) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
}

fn get_token_usdc(e: &Env) -> Address {
    e.storage().get_unchecked(&DataKey::TokenUSDC).unwrap()
}

fn get_token_share(e: &Env) -> Address {
    e.storage().get_unchecked(&DataKey::TokenShare).unwrap()
}

fn get_total_shares(e: &Env) -> i128 {
    e.storage().get_unchecked(&DataKey::TotalShares).unwrap()
}

fn get_reserve_usdc(e: &Env) -> i128 {
    e.storage().get_unchecked(&DataKey::ReserveUSDC).unwrap()
}

fn get_balance(e: &Env, contract: Address) -> i128 {
    token::Client::new(e, &contract).balance(&e.current_contract_address())
}

fn get_balance_usdc(e: &Env) -> i128 {
    get_balance(e, get_token_usdc(e))
}

fn get_balance_shares(e: &Env) -> i128 {
    get_balance(e, get_token_share(e))
}

fn put_token_usdc(e: &Env, contract: Address) {
    e.storage().set(&DataKey::TokenUSDC, &contract);
}

fn put_token_share(e: &Env, contract: Address) {
    e.storage().set(&DataKey::TokenShare, &contract);
}

fn put_total_shares(e: &Env, amount: i128) {
    e.storage().set(&DataKey::TotalShares, &amount)
}

fn put_reserve_usdc(e: &Env, amount: i128) {
    e.storage().set(&DataKey::ReserveUSDC, &amount)
}

fn burn_shares(e: &Env, amount: i128) {
    let total = get_total_shares(e);
    let share_contract = get_token_share(e);

    token::Client::new(e, &share_contract).burn(&e.current_contract_address(), &amount);
    put_total_shares(e, total - amount);
}

fn mint_shares(e: &Env, to: Address, amount: i128) {
    let total = get_total_shares(e);
    let share_contract_id = get_token_share(e);

    token::Client::new(e, &share_contract_id).mint(&to, &amount);

    put_total_shares(e, total + amount);
}

fn transfer(e: &Env, token: Address, to: Address, amount: i128) {
    token::Client::new(e, &token).transfer(&e.current_contract_address(), &to, &amount);
}

fn transfer_usdc(e: &Env, to: Address, amount: i128) {
    transfer(e, get_token_usdc(e), to, amount);
}

// Metadata that is added on to the WASM custom section
contractmeta!(key = "Description", val = "Money market product tokenizer");

pub trait LendCraftTokenizerTrait {
    fn initialize(e: Env, token_wasm_hash: BytesN<32>, token_usdc: Address);

    fn share_id(e: Env) -> Address;

    fn deposit(e: Env, to: Address, usdc_amount: i128) -> Result<i128, LendCraftError>;

    fn withdraw(e: Env, to: Address, share_amount: i128) -> Result<i128, LendCraftError>;
}

struct LendCraftTokenizer;

#[contractimpl]
impl LendCraftTokenizerTrait for LendCraftTokenizer {
    fn initialize(e: Env, token_wasm_hash: BytesN<32>, token_usdc: Address) {
        let share_contract = create_contract(&e, &token_wasm_hash, &token_usdc);
        token::Client::new(&e, &share_contract).initialize(
            &e.current_contract_address(),
            &7u32,
            &Bytes::from_slice(&e, b"Lend Craft Token"),
            &Bytes::from_slice(&e, b"LCT"),
        );

        put_token_usdc(&e, token_usdc);
        put_token_share(&e, share_contract.try_into().unwrap());
        put_total_shares(&e, 0);
        put_reserve_usdc(&e, 0);
    }

    fn share_id(e: Env) -> Address {
        get_token_share(&e)
    }

    fn deposit(e: Env, to: Address, usdc_deposit: i128) -> Result<i128, LendCraftError> {
        to.require_auth();

        let zero = 0;
        if usdc_deposit <= zero {
            return Err(LendCraftError::DepositMustBePositive);
        }
        let reserve_usdc = get_reserve_usdc(&e);
        let token_usdc_client = token::Client::new(&e, &get_token_usdc(&e));
        token_usdc_client.transfer(&to, &e.current_contract_address(), &usdc_deposit);

        let balance_usdc = get_balance_usdc(&e);
        let total_shares = get_total_shares(&e);

        let new_total_shares = if reserve_usdc > zero {
            (balance_usdc * total_shares) / reserve_usdc
        } else {
            balance_usdc
        };

        mint_shares(&e, to, new_total_shares - total_shares);
        put_reserve_usdc(&e, balance_usdc);

        Ok(new_total_shares)
    }

    fn withdraw(e: Env, to: Address, share_amount: i128) -> Result<i128, LendCraftError> {
        to.require_auth();
        let share_token_client = token::Client::new(&e, &get_token_share(&e));

        if share_amount <= 0 {
            return Err(LendCraftError::WithdrawalMustBePositive);
        }

        if share_amount > share_token_client.balance(&to) {
            return Err(LendCraftError::InsufficientBalance);
        }

        share_token_client.transfer(&to, &e.current_contract_address(), &share_amount);

        let balance_usdc = get_balance_usdc(&e);
        let balance_shares = get_balance_shares(&e);
        let total_shares = get_total_shares(&e);

        let out_usdc = (balance_usdc * balance_shares) / total_shares;

        burn_shares(&e, balance_shares);
        transfer_usdc(&e, to.clone(), out_usdc);
        put_reserve_usdc(&e, balance_usdc - out_usdc);

        Ok(out_usdc)
    }
}
