#![cfg(test)]

use super::*;

use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_store() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_address = env.register_contract(None, DataStoreContract);
    let client = DataStoreContractClient::new(&env, &contract_address);
    let u1 = Address::random(&env);
    let u2 = Address::random(&env);
    client.put(&u1, &bytes!(&env, 0x48656c6c6f20536f726f62616e21)); // This is the hex value for "Hello Soroban!"
    assert_eq!(
        client.get(&u1),
        bytes!(&env, 0x48656c6c6f20536f726f62616e21)
    );

    assert_eq!(client.get(&u2).len(), 0);
    client.put(&u2, &bytes![&env, 0x536f726f62616e2051756573742032]); // This is the hex value for "Soroban Quest 2"
    assert_eq!(
        client.get(&u2),
        bytes![&env, 0x536f726f62616e2051756573742032]
    );

    assert_eq!(
        client.get(&u1),
        bytes![&env, 0x48656c6c6f20536f726f62616e21]
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(2))")]
fn test_store_value_too_short() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_address = env.register_contract(None, DataStoreContract);
    let client = DataStoreContractClient::new(&env, &contract_address);
    let u1 = Address::random(&env);
    client.put(&u1, &bytes![&env, 0x007]);
}

pub struct CallerContract;

#[contractimpl]
impl CallerContract {
    pub fn try_put(env: Env, contract_address: Address, user: Address, data: Bytes) {
        let cli = DataStoreContractClient::new(&env, &contract_address);
        cli.put(&user, &data);
    }

    pub fn try_get(env: Env, contract_address: Address, owner: Address) -> Bytes {
        let cli = DataStoreContractClient::new(&env, &contract_address);
        cli.get(&owner)
    }
}

#[test]
fn test_contract_store() {
    let env = Env::default();

    env.mock_all_auths();

    let contract_address_data_store = env.register_contract(None, DataStoreContract);
    let data_store_client = DataStoreContractClient::new(&env, &contract_address_data_store);

    let contract_address_caller = env.register_contract(None, CallerContract);
    let caller_client = CallerContractClient::new(&env, &contract_address_caller);

    caller_client.try_put(
        &contract_address_data_store,
        &contract_address_caller,
        &bytes![&env, 0x48656c6c6f20536f726f62616e21],
    );

    assert_eq!(
        data_store_client.get(&contract_address_caller),
        bytes![&env, 0x48656c6c6f20536f726f62616e21]
    );
}

#[test]
fn test_contract_get() {
    let env = Env::default();

    env.mock_all_auths();

    let contract_address_data_store = env.register_contract(None, DataStoreContract);
    let client_data_store = DataStoreContractClient::new(&env, &contract_address_data_store);

    let contract_address_caller = env.register_contract(None, CallerContract);
    let caller_client = CallerContractClient::new(&env, &contract_address_caller);

    let u1 = Address::random(&env);
    client_data_store.put(&u1, &bytes!(&env, 0x48656c6c6f20536f726f62616e21)); // This is the hex value for "Hello Soroban!"

    let value = caller_client.try_get(&contract_address_data_store, &u1);
    assert_eq!(value, bytes!(&env, 0x48656c6c6f20536f726f62616e21));
}
