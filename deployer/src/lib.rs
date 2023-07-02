#![no_std]

use soroban_sdk::{contractimpl, Address, BytesN, Env, RawVal, Symbol, Vec};

pub struct Deployer;

#[contractimpl]
impl Deployer {
    pub fn deploy(
        env: Env,
        salt: BytesN<32>,
        wasm_hash: BytesN<32>,
        init_fn: Symbol,
        init_args: Vec<RawVal>,
    ) -> (Address, RawVal) {
        let id = env
            .deployer()
            .with_current_contract(&salt)
            .deploy(&wasm_hash);
        let res: RawVal = env.invoke_contract(&id, &init_fn, init_args);
        (id, res)
    }
}

mod test;
