use crate::locals::Error;
use clap::{command, Parser};
use soroban_cli::{commands, fee, rpc, utils, wasm};
use soroban_sdk::xdr::{
    Error as XdrError, Hash, HostFunction, HostFunctionArgs, InvokeHostFunctionOp, Memo,
    MuxedAccount, Operation, OperationBody, Preconditions, SequenceNumber, Transaction,
    TransactionExt, Uint256, UploadContractWasmArgs, VecM,
};

#[derive(Parser, Debug, Clone)]
#[group(skip)]
pub struct UploadCmd {
    #[command(flatten)]
    pub config: commands::config::Args,
    #[command(flatten)]
    pub fee: fee::Args,
    #[command(flatten)]
    pub wasm: wasm::Args,
}

impl UploadCmd {
    pub async fn run(&self) -> Result<(), Error> {
        let res_str = hex::encode(self.run_and_get_hash().await?);
        println!("{res_str}");
        Ok(())
    }

    pub async fn run_and_get_hash(&self) -> Result<Hash, Error> {
        let contract = self.wasm.read()?;
        if self.config.is_no_network() {
            self.run_in_sandbox(contract)
        } else {
            self.run_against_rpc_server(contract).await
        }
    }

    pub fn run_in_sandbox(&self, contract: Vec<u8>) -> Result<Hash, Error> {
        let mut state = self.config.get_state()?;
        let wasm_hash =
            utils::add_contract_code_to_ledger_entries(&mut state.ledger_entries, contract)?;

        self.config.set_state(&mut state)?;

        Ok(wasm_hash)
    }

    async fn run_against_rpc_server(&self, contract: Vec<u8>) -> Result<Hash, Error> {
        let network = self.config.get_network()?;
        let client = rpc::Client::new(&network.rpc_url)?;
        let key = self.config.key_pair()?;

        // Get the account sequence number
        let public_strkey = stellar_strkey::ed25519::PublicKey(key.public.to_bytes()).to_string();
        let account_details = client.get_account(&public_strkey).await?;
        let sequence: i64 = account_details.seq_num.into();

        let (tx_without_preflight, hash) =
            build_install_contract_code_tx(contract.clone(), sequence + 1, self.fee.fee, &key)?;

        client
            .prepare_and_send_transaction(
                &tx_without_preflight,
                &key,
                &network.network_passphrase,
                None,
            )
            .await?;

        Ok(hash)
    }
}

pub(crate) fn build_install_contract_code_tx(
    contract: Vec<u8>,
    sequence: i64,
    fee: u32,
    key: &ed25519_dalek::Keypair,
) -> Result<(Transaction, Hash), XdrError> {
    let hash = utils::contract_hash(&contract)?;

    let op = Operation {
        source_account: Some(MuxedAccount::Ed25519(Uint256(key.public.to_bytes()))),
        body: OperationBody::InvokeHostFunction(InvokeHostFunctionOp {
            functions: vec![HostFunction {
                args: HostFunctionArgs::UploadContractWasm(UploadContractWasmArgs {
                    code: contract.try_into()?,
                }),
                auth: VecM::default(),
            }]
            .try_into()?,
        }),
    };

    let tx = Transaction {
        source_account: MuxedAccount::Ed25519(Uint256(key.public.to_bytes())),
        fee,
        seq_num: SequenceNumber(sequence),
        cond: Preconditions::None,
        memo: Memo::None,
        operations: vec![op].try_into()?,
        ext: TransactionExt::V0,
    };

    Ok((tx, hash))
}
