use crate::error::ExcellarError;
use crate::storage;
use crate::token;
use crate::utils;
use soroban_sdk::{contract, contractimpl, contractmeta, Address, BytesN, Env, IntoVal};
use storage::{
    get_cash_reserves, get_etf_market_value, get_fees, get_token_usdc, get_token_xusg,
    get_total_xusg, require_admin, set_admin, set_cash_reserves, set_etf_market_value, set_fees,
    set_token_usdc, set_token_xusg, set_total_xusg,
};

use crate::storage::{add_to_cash_reserves, get_balance_usdc, subtract_from_cash_reserves};
use token::create_contract;
use utils::{require_positive, require_strictly_positive};

fn burn_xusg(e: &Env, amount: i128) {
    let total = get_total_xusg(e);
    let xusg_contract = get_token_xusg(e);

    token::Client::new(e, &xusg_contract).burn(&e.current_contract_address(), &amount);
    set_total_xusg(e, total - amount);
}

fn mint_xusg(e: &Env, to: Address, amount: i128) {
    let total = get_total_xusg(e);
    let xusg_contract_id = get_token_xusg(e);

    token::Client::new(e, &xusg_contract_id).mint(&to, &amount);

    set_total_xusg(e, total + amount);
}

fn transfer(e: &Env, token: Address, to: Address, amount: i128) {
    token::Client::new(e, &token).transfer(&e.current_contract_address(), &to, &amount);
}

fn transfer_usdc(e: &Env, to: Address, amount: i128) {
    transfer(e, get_token_usdc(e), to, amount);
}

fn calculate_xusg_price(e: &Env) -> i128 {
    let cash_reserves = get_cash_reserves(e);
    let fees = get_fees(e);
    let etf_market_value = get_etf_market_value(e);
    let total_xusg = get_total_xusg(e);

    if total_xusg <= 0 {
        return 1;
    }

    if etf_market_value + cash_reserves - fees == 0 {
        return 1;
    }

    (etf_market_value + cash_reserves - fees) / total_xusg
}

// Metadata that is added on to the WASM custom section
contractmeta!(key = "Description", val = "Money market product tokenizer");

pub trait ExcellarTokenizerTrait {
    fn initialize(e: Env, token_wasm_hash: BytesN<32>, token_usdc: Address, admin: Address);

    fn xusg_id(e: Env) -> Address;

    fn etf_market_value(e: Env) -> i128;

    fn set_etf_market_value(e: Env, price: i128);

    fn cash_reserves(e: Env) -> i128;

    fn set_cash_reserves(e: Env, amount: i128);

    fn fees(e: Env) -> i128;

    fn set_fees(e: Env, amount: i128);

    fn deposit(e: Env, to: Address, usdc_amount: i128) -> Result<i128, ExcellarError>;

    fn withdraw(e: Env, to: Address, xusg_amount: i128) -> Result<i128, ExcellarError>;

    fn withdraw_admin(e: Env, to: Address, usdc_amount: i128) -> Result<i128, ExcellarError>;

    fn balance(e: Env, account: Address) -> i128;

    fn price(e: Env) -> i128;

    fn total(e: Env) -> i128;
}

#[contract]
pub struct ExcellarTokenizer;

#[contractimpl]
impl ExcellarTokenizerTrait for ExcellarTokenizer {
    fn initialize(e: Env, token_wasm_hash: BytesN<32>, token_usdc: Address, admin: Address) {
        let xusg_contract = create_contract(&e, token_wasm_hash, &token_usdc);
        token::Client::new(&e, &xusg_contract).initialize(
            &e.current_contract_address(),
            &7,
            &"Excellar Mint".into_val(&e),
            &"XUSG".into_val(&e),
        );

        set_token_usdc(&e, token_usdc);
        set_token_xusg(&e, xusg_contract);
        set_total_xusg(&e, 0);
        set_cash_reserves(&e, 0);
        set_fees(&e, 0);
        set_etf_market_value(&e, 0);
        set_admin(&e, admin);
    }

    fn xusg_id(e: Env) -> Address {
        get_token_xusg(&e)
    }

    fn etf_market_value(e: Env) -> i128 {
        get_etf_market_value(&e)
    }

    fn set_etf_market_value(e: Env, value: i128) {
        require_admin(&e);
        require_strictly_positive(value);
        set_etf_market_value(&e, value);
        require_strictly_positive(calculate_xusg_price(&e));
    }

    fn cash_reserves(e: Env) -> i128 {
        get_cash_reserves(&e)
    }

    fn set_cash_reserves(e: Env, amount: i128) {
        require_admin(&e);
        require_positive(amount);
        set_cash_reserves(&e, amount);
        require_strictly_positive(calculate_xusg_price(&e));
    }

    fn fees(e: Env) -> i128 {
        get_fees(&e)
    }

    fn set_fees(e: Env, amount: i128) {
        require_admin(&e);
        require_positive(amount);
        set_fees(&e, amount);
        require_strictly_positive(calculate_xusg_price(&e));
    }

    fn balance(e: Env, account: Address) -> i128 {
        let xusg_contract = get_token_xusg(&e);
        token::Client::new(&e, &xusg_contract).balance(&account)
    }

    fn price(e: Env) -> i128 {
        calculate_xusg_price(&e)
    }

    fn total(e: Env) -> i128 {
        get_total_xusg(&e)
    }

    fn deposit(e: Env, to: Address, usdc_deposit: i128) -> Result<i128, ExcellarError> {
        to.require_auth();

        let zero = 0;
        if usdc_deposit <= zero {
            return Err(ExcellarError::DepositMustBePositive);
        }
        let token_usdc_token = token::Client::new(&e, &get_token_usdc(&e));
        token_usdc_token.transfer(&to, &e.current_contract_address(), &usdc_deposit);

        let xusg_price = calculate_xusg_price(&e);
        let xusg_issued = usdc_deposit / xusg_price;
        add_to_cash_reserves(&e, usdc_deposit);

        mint_xusg(&e, to, xusg_issued);

        Ok(xusg_issued)
    }

    fn withdraw(e: Env, to: Address, xusg_amount: i128) -> Result<i128, ExcellarError> {
        to.require_auth();
        let token_usdc_token = token::Client::new(&e, &get_token_xusg(&e));

        if xusg_amount <= 0 {
            return Err(ExcellarError::WithdrawalMustBePositive);
        }

        if xusg_amount > token_usdc_token.balance(&to) {
            return Err(ExcellarError::InsufficientBalance);
        }

        token_usdc_token.transfer(&to, &e.current_contract_address(), &xusg_amount);

        let xusg_price = calculate_xusg_price(&e);
        let out_usdc = xusg_amount * xusg_price;

        subtract_from_cash_reserves(&e, out_usdc);
        burn_xusg(&e, xusg_amount);

        transfer_usdc(&e, to, out_usdc);

        Ok(out_usdc)
    }

    fn withdraw_admin(e: Env, to: Address, usdc_amount: i128) -> Result<i128, ExcellarError> {
        require_admin(&e);
        let zero = 0;
        if usdc_amount <= zero {
            return Err(ExcellarError::WithdrawalMustBePositive);
        }

        let balance_cash = get_balance_usdc(&e);
        if usdc_amount > balance_cash {
            return Err(ExcellarError::InsufficientBalance);
        }

        transfer_usdc(&e, to, usdc_amount);
        Ok(usdc_amount)
    }
}
