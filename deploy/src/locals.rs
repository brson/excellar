use soroban_cli::{commands, rpc, wasm};
use soroban_sdk::xdr::Error as XdrError;
use std::array::TryFromSliceError;
use std::fmt::Debug;
use std::num::ParseIntError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("cannot parse salt: {salt}")]
    CannotParseSalt { salt: String },
    #[error("cannot parse contract ID {contract_id}: {error}")]
    CannotParseContractId {
        contract_id: String,
        error: stellar_strkey::DecodeError,
    },
    #[error("cannot parse WASM hash {wasm_hash}: {error}")]
    CannotParseWasmHash {
        wasm_hash: String,
        error: stellar_strkey::DecodeError,
    },
    #[error("Must provide either --wasm or --wash-hash")]
    WasmNotProvided,
    #[error("error parsing int: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("internal conversion error: {0}")]
    TryFromSliceError(#[from] TryFromSliceError),
    #[error("xdr processing error: {0}")]
    Xdr(#[from] XdrError),
    #[error(transparent)]
    Rpc(#[from] rpc::Error),
    #[error(transparent)]
    Config(#[from] commands::config::Error),
    #[error(transparent)]
    Wasm(#[from] wasm::Error),
    #[error("unexpected ({length}) simulate transaction result length")]
    UnexpectedSimulateTransactionResultSize { length: usize },
}
