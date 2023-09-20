use crate::utils::require_positive;
use soroban_sdk::{token, Address, ConversionError, Env, TryFromVal, Val};

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum DataKey {
    TokenUSDC = 1,
    TokenXUSG = 2,
    TotalXUSG = 3,
    MarketETFPrice = 5,
    ReservesCash = 6,
    Admin = 7,
    Fees = 8,
}
impl TryFromVal<Env, DataKey> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &DataKey) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
}

pub fn get_etf_market_value(e: &Env) -> i128 {
    e.storage()
        .persistent()
        .get(&DataKey::MarketETFPrice)
        .unwrap()
}

pub fn get_token_usdc(e: &Env) -> Address {
    e.storage().persistent().get(&DataKey::TokenUSDC).unwrap()
}

pub fn get_token_xusg(e: &Env) -> Address {
    e.storage().persistent().get(&DataKey::TokenXUSG).unwrap()
}

pub fn get_total_xusg(e: &Env) -> i128 {
    e.storage().persistent().get(&DataKey::TotalXUSG).unwrap()
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
    e.storage()
        .persistent()
        .get(&DataKey::ReservesCash)
        .unwrap()
}

pub fn get_fees(e: &Env) -> i128 {
    e.storage().persistent().get(&DataKey::Fees).unwrap()
}

pub fn get_balance(e: &Env, contract: Address) -> i128 {
    token::Client::new(e, &contract).balance(&e.current_contract_address())
}

pub fn get_balance_usdc(e: &Env) -> i128 {
    get_balance(e, get_token_usdc(e))
}

pub fn set_token_usdc(e: &Env, contract: Address) {
    e.storage().persistent().set(&DataKey::TokenUSDC, &contract);
}

pub fn set_token_xusg(e: &Env, contract: Address) {
    e.storage().persistent().set(&DataKey::TokenXUSG, &contract);
}

pub fn set_total_xusg(e: &Env, amount: i128) {
    e.storage().persistent().set(&DataKey::TotalXUSG, &amount)
}

pub fn set_cash_reserves(e: &Env, amount: i128) {
    e.storage()
        .persistent()
        .set(&DataKey::ReservesCash, &amount)
}

pub fn set_etf_market_value(e: &Env, price: i128) {
    e.storage()
        .persistent()
        .set(&DataKey::MarketETFPrice, &price)
}

pub fn set_fees(e: &Env, price: i128) {
    e.storage().persistent().set(&DataKey::Fees, &price)
}

pub fn set_admin(e: &Env, admin: Address) {
    e.storage().persistent().set(&DataKey::Admin, &admin)
}

pub fn require_admin(e: &Env) {
    e.storage()
        .persistent()
        .get::<DataKey, Address>(&DataKey::Admin)
        .unwrap()
        .require_auth();
}
