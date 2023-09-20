use clap::{arg, command, Parser};
use locals::Error;
use rand::Rng;
use sha2::{Digest, Sha256};
use soroban_cli::{commands, fee, rpc, utils, wasm};
use soroban_sdk::xdr::{AccountId, ContractId, CreateContractArgs, DiagnosticEvent, Hash, HashIdPreimage, HashIdPreimageSourceAccountContractId, HostFunction, HostFunctionArgs, InvokeHostFunctionOp, Memo, MuxedAccount, Operation, OperationBody, Preconditions, PublicKey, ScContractExecutable, SequenceNumber, Transaction, TransactionExt, TransactionResult, Uint256, VecM, WriteXdr};

use upload::UploadCmd;

mod locals;
mod upload;

#[derive(Parser, Debug, Clone)]
#[command(group(
clap::ArgGroup::new("wasm_src")
.required(true)
.args(& ["wasm", "wasm_hash"]),
))]
#[group(skip)]
pub struct Cmd {
    #[arg(long, group = "wasm_src")]
    wasm: Option<std::path::PathBuf>,
    #[arg(long = "wasm-hash", conflicts_with = "wasm", group = "wasm_src")]
    wasm_hash: Option<String>,
    #[arg(
    long = "id",
    conflicts_with = "rpc_url",
    help_heading = commands::HEADING_SANDBOX,
    )]
    contract_id: Option<String>,
    #[arg(
    long,
    conflicts_with_all = & ["contract_id", "ledger_file"],
    help_heading = commands::HEADING_RPC,
    )]
    salt: Option<String>,
    #[command(flatten)]
    config: commands::config::Args,
    #[command(flatten)]
    pub fee: fee::Args,
}

impl Cmd {
    pub async fn run(&self) -> Result<(), Error> {
        let res_str = self.run_and_get_contract_id().await?;
        println!("{res_str}");
        Ok(())
    }

    pub async fn run_and_get_contract_id(&self) -> Result<String, Error> {
        let wasm_hash = if let Some(wasm) = &self.wasm {
            let hash = UploadCmd {
                wasm: wasm::Args { wasm: wasm.clone() },
                config: self.config.clone(),
                fee: self.fee.clone(),
            }
            .run_and_get_hash()
            .await?;
            hex::encode(hash)
        } else {
            self.wasm_hash
                .as_ref()
                .ok_or(Error::WasmNotProvided)?
                .to_string()
        };

        let hash = Hash(utils::contract_id_from_str(&wasm_hash).map_err(|e| {
            Error::CannotParseWasmHash {
                wasm_hash: wasm_hash.clone(),
                error: e,
            }
        })?);

        if self.config.is_no_network() {
            self.run_in_sandbox(hash)
        } else {
            self.run_against_rpc_server(hash).await
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn run_in_sandbox(&self, wasm_hash: Hash) -> Result<String, Error> {
        let contract_id: [u8; 32] = match &self.contract_id {
            Some(id) => {
                utils::contract_id_from_str(id).map_err(|e| Error::CannotParseContractId {
                    contract_id: self.contract_id.as_ref().unwrap().clone(),
                    error: e,
                })?
            }
            None => rand::thread_rng().gen::<[u8; 32]>(),
        };

        let mut state = self.config.get_state()?;
        utils::add_contract_to_ledger_entries(&mut state.ledger_entries, contract_id, wasm_hash.0);
        self.config.set_state(&mut state)?;
        Ok(stellar_strkey::Contract(contract_id).to_string())
    }

    async fn run_against_rpc_server(&self, wasm_hash: Hash) -> Result<String, Error> {
        let network = self.config.get_network()?;
        let salt: [u8; 32] = match &self.salt {
            Some(h) => soroban_spec_tools::utils::padded_hex_from_str(h, 32)
                .map_err(|_| Error::CannotParseSalt { salt: h.clone() })?
                .try_into()
                .map_err(|_| Error::CannotParseSalt { salt: h.clone() })?,
            None => rand::thread_rng().gen::<[u8; 32]>(),
        };

        let client = rpc::Client::new(&network.rpc_url)?;
        let key = self.config.key_pair()?;

        // Get the account sequence number
        let public_strkey = stellar_strkey::ed25519::PublicKey(key.public.to_bytes()).to_string();

        let account_details = client.get_account(&public_strkey).await?;
        let sequence: i64 = account_details.seq_num.into();
        let (tx, contract_id) = build_create_contract_tx(
            wasm_hash,
            sequence + 1,
            self.fee.fee,
            &network.network_passphrase,
            salt,
            &key,
        )?;
        let res = client
            .prepare_and_send_transaction(&tx, &key, &network.network_passphrase, None)
            .await;
        match res {
            Ok((res, diag)) => {
                println!("result {:?}",res.result);
            }
            Err(e) => {
                println!("problem {}", e)
            }
        }
        Ok(stellar_strkey::Contract(contract_id.0).to_string())
    }
}

fn build_create_contract_tx(
    hash: Hash,
    sequence: i64,
    fee: u32,
    network_passphrase: &str,
    salt: [u8; 32],
    key: &ed25519_dalek::Keypair,
) -> Result<(Transaction, Hash), Error> {
    let network_id = Hash(Sha256::digest(network_passphrase.as_bytes()).into());
    let preimage =
        HashIdPreimage::ContractIdFromSourceAccount(HashIdPreimageSourceAccountContractId {
            network_id,
            source_account: AccountId(PublicKey::PublicKeyTypeEd25519(
                key.public.to_bytes().into(),
            )),
            salt: Uint256(salt),
        });
    let preimage_xdr = preimage.to_xdr()?;
    let contract_id = Sha256::digest(preimage_xdr);

    let op = Operation {
        source_account: None,
        body: OperationBody::InvokeHostFunction(InvokeHostFunctionOp {
            functions: vec![HostFunction {
                args: HostFunctionArgs::CreateContract(CreateContractArgs {
                    contract_id: ContractId::SourceAccount(Uint256(salt)),
                    executable: ScContractExecutable::WasmRef(hash),
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

    Ok((tx, Hash(contract_id.into())))
}

fn main() {
    let cmd = Cmd::parse();
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(cmd.run()).unwrap();
}
