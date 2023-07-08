use crate::error::ExcellarError;
use crate::storage;
use crate::token;
use crate::utils;
use soroban_sdk::{contractimpl, contractmeta, Address, Bytes, BytesN, Env};
use storage::{
    get_cash_reserves, get_etf_price, get_fees, get_token_musg, get_token_usdc, get_total_musg,
    require_admin, set_admin, set_cash_reserves, set_etf_price, set_fees, set_token_musg,
    set_token_usdc, set_total_musg,
};

use crate::storage::{add_to_cash_reserves, get_balance_usdc, subtract_from_cash_reserves};
use token::create_contract;
use utils::{require_positive, require_strictly_positive};

fn burn_musg(e: &Env, amount: i128) {
    let total = get_total_musg(e);
    let musg_contract = get_token_musg(e);

    token::Client::new(e, &musg_contract).burn(&e.current_contract_address(), &amount);
    set_total_musg(e, total - amount);
}

fn mint_musg(e: &Env, to: Address, amount: i128) {
    let total = get_total_musg(e);
    let musg_contract_id = get_token_musg(e);

    token::Client::new(e, &musg_contract_id).mint(&to, &amount);

    set_total_musg(e, total + amount);
}

fn transfer(e: &Env, token: Address, to: Address, amount: i128) {
    token::Client::new(e, &token).transfer(&e.current_contract_address(), &to, &amount);
}

fn transfer_usdc(e: &Env, to: Address, amount: i128) {
    transfer(e, get_token_usdc(e), to, amount);
}

fn calculate_musg_price(e: &Env) -> i128 {
    let cash_reserves = get_cash_reserves(e);
    let fees = get_fees(e);
    let etf_price = get_etf_price(e);
    let total_musg = get_total_musg(e);

    if total_musg <= 0 {
        return 1;
    }

    (etf_price + cash_reserves + fees) / total_musg
}

// Metadata that is added on to the WASM custom section
contractmeta!(key = "Description", val = "Money market product tokenizer");

pub trait ExcellarTokenizerTrait {
    fn initialize(e: Env, token_wasm_hash: BytesN<32>, token_usdc: Address, admin: Address);

    fn musg_id(e: Env) -> Address;

    fn etf_price(e: Env) -> i128;

    fn set_etf_price(e: Env, price: i128);

    fn cash_reserves(e: Env) -> i128;

    fn set_cash_reserves(e: Env, amount: i128);

    fn fees(e: Env) -> i128;

    fn set_fees(e: Env, amount: i128);

    fn deposit(e: Env, to: Address, usdc_amount: i128) -> Result<i128, ExcellarError>;

    fn withdraw(e: Env, to: Address, musg_amount: i128) -> Result<i128, ExcellarError>;

    fn withdraw_admin(e: Env, to: Address, usdc_amount: i128) -> Result<i128, ExcellarError>;
}

pub struct ExcellarTokenizer;

#[contractimpl]
impl ExcellarTokenizerTrait for ExcellarTokenizer {
    fn initialize(e: Env, token_wasm_hash: BytesN<32>, token_usdc: Address, admin: Address) {
        let musg_contract = create_contract(&e, &token_wasm_hash, &token_usdc);
        token::Client::new(&e, &musg_contract).initialize(
            &e.current_contract_address(),
            &7u32,
            &Bytes::from_slice(&e, b"Excellar Mint"),
            &Bytes::from_slice(&e, b"mUSG"),
        );

        set_token_usdc(&e, token_usdc);
        set_token_musg(&e, musg_contract);
        set_total_musg(&e, 0);
        set_cash_reserves(&e, 0);
        set_cash_reserves(&e, 0);
        set_fees(&e, 0);
        set_etf_price(&e, 1);
        set_admin(&e, admin);
    }

    fn musg_id(e: Env) -> Address {
        get_token_musg(&e)
    }

    fn etf_price(e: Env) -> i128 {
        get_etf_price(&e)
    }

    fn set_etf_price(e: Env, price: i128) {
        require_admin(&e);
        require_strictly_positive(price);
        set_etf_price(&e, price)
    }

    fn cash_reserves(e: Env) -> i128 {
        get_cash_reserves(&e)
    }

    fn set_cash_reserves(e: Env, amount: i128) {
        require_admin(&e);
        require_positive(amount);
        set_cash_reserves(&e, amount)
    }

    fn fees(e: Env) -> i128 {
        get_fees(&e)
    }

    fn set_fees(e: Env, amount: i128) {
        require_admin(&e);
        require_positive(amount);
        set_fees(&e, amount)
    }

    fn deposit(e: Env, to: Address, usdc_deposit: i128) -> Result<i128, ExcellarError> {
        to.require_auth();

        let zero = 0;
        if usdc_deposit <= zero {
            return Err(ExcellarError::DepositMustBePositive);
        }
        let token_usdc_client = token::Client::new(&e, &get_token_usdc(&e));
        token_usdc_client.transfer(&to, &e.current_contract_address(), &usdc_deposit);

        let musg_price = calculate_musg_price(&e);
        let musg_issued = usdc_deposit / musg_price;
        add_to_cash_reserves(&e, usdc_deposit);

        mint_musg(&e, to, musg_issued);

        Ok(musg_issued)
    }

    fn withdraw(e: Env, to: Address, musg_amount: i128) -> Result<i128, ExcellarError> {
        to.require_auth();
        let musg_token_client = token::Client::new(&e, &get_token_musg(&e));

        if musg_amount <= 0 {
            return Err(ExcellarError::WithdrawalMustBePositive);
        }

        if musg_amount > musg_token_client.balance(&to) {
            return Err(ExcellarError::InsufficientBalance);
        }

        musg_token_client.transfer(&to, &e.current_contract_address(), &musg_amount);

        let musg_price = calculate_musg_price(&e);
        let out_usdc = musg_amount * musg_price;

        subtract_from_cash_reserves(&e, out_usdc);
        burn_musg(&e, musg_amount);

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
