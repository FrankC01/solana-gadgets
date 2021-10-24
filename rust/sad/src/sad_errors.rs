//! @brief Data map

use solana_client::client_error::ClientError;
use solana_sdk::pubkey::Pubkey;
use std::{io, result::Result as GenericResult};
use thiserror::Error;

/// sad application error types
pub type SadBaseResult = GenericResult<(), SadAppError>;
pub type SadTypeResult<T> = GenericResult<T, SadAppError>;

#[derive(Error, Debug)]
pub enum SadAppError {
    #[error("IO error occured ")]
    IoError(#[from] io::Error),
    #[error("\n{0}\n")]
    ConnectionError(ClientError),
    #[error("DataMapping type {key} has no 'type' declared")]
    DataMappingMissingTypeError { key: String },
    #[error("DataMapping key {key} type {value} unknown")]
    DataMappingError { key: String, value: String },
    #[error("Account {0} not found ")]
    NoAccount(Pubkey),
    #[error("Account {0} is an executable account ")]
    AccountIsExecutable(Pubkey),
    #[error("Account {0} has no data ")]
    AccountHasNoData(Pubkey),
}
