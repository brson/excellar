use crate::utils::require_positive;
use soroban_sdk::{token, Address, ConversionError, Env, RawVal, TryFromVal};

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum DataKey {
    TokenUSDC = 1,
    TokenMUSG = 2,
    TotalMUSG = 3,
    MarketETFPrice = 5,
    ReservesCash = 6,
    Admin = 7,
    Fees = 8,
}

impl TryFromVal<Env, DataKey> for RawVal {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &DataKey) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
}

pub fn get_etf_price(e: &Env) -> i128 {
    e.storage().get_unchecked(&DataKey::MarketETFPrice).unwrap()
}

pub fn get_token_usdc(e: &Env) -> Address {
    e.storage().get_unchecked(&DataKey::TokenUSDC).unwrap()
}

pub fn get_token_musg(e: &Env) -> Address {
    e.storage().get_unchecked(&DataKey::TokenMUSG).unwrap()
}

pub fn get_total_musg(e: &Env) -> i128 {
    e.storage().get_unchecked(&DataKey::TotalMUSG).unwrap()
}

pub fn add_to_cash_reserves(e: &Env, amount: i128) -> i128 {
    require_positive(amount);
    let cash_reserves = get_cash_reserves(e);
    let new_cash = cash_reserves + amount;
    set_cash_reserves(e, new_cash);
    new_cash
}
pub fn subtract_from_cash_reserves(e: &Env, amount: i128) -> i128 {
    require_positive(amount);
    let cash_reserves = get_cash_reserves(e);
    let new_cash = cash_reserves - amount;
    set_cash_reserves(e, cash_reserves - amount);
    new_cash
}
pub fn get_cash_reserves(e: &Env) -> i128 {
    e.storage().get_unchecked(&DataKey::ReservesCash).unwrap()
}

pub fn get_fees(e: &Env) -> i128 {
    e.storage().get_unchecked(&DataKey::Fees).unwrap()
}

pub fn get_balance(e: &Env, contract: Address) -> i128 {
    token::Client::new(e, &contract).balance(&e.current_contract_address())
}

pub fn get_balance_usdc(e: &Env) -> i128 {
    get_balance(e, get_token_usdc(e))
}

pub fn set_token_usdc(e: &Env, contract: Address) {
    e.storage().set(&DataKey::TokenUSDC, &contract);
}

pub fn set_token_musg(e: &Env, contract: Address) {
    e.storage().set(&DataKey::TokenMUSG, &contract);
}

pub fn set_total_musg(e: &Env, amount: i128) {
    e.storage().set(&DataKey::TotalMUSG, &amount)
}

pub fn set_cash_reserves(e: &Env, amount: i128) {
    e.storage().set(&DataKey::ReservesCash, &amount)
}

pub fn set_etf_price(e: &Env, price: i128) {
    e.storage().set(&DataKey::MarketETFPrice, &price)
}

pub fn set_fees(e: &Env, price: i128) {
    e.storage().set(&DataKey::Fees, &price)
}

pub fn set_admin(e: &Env, admin: Address) {
    e.storage().set(&DataKey::Admin, &admin)
}

pub fn require_admin(e: &Env) {
    e.storage()
        .get::<DataKey, Address>(&DataKey::Admin)
        .unwrap()
        .expect("Admin required")
        .require_auth();
}
