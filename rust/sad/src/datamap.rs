//! @brief Data map

use gadgets_common::load_yaml_file;
use gadgets_common::sol_txns::account_for_key;
use serde::Deserialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::{collections::HashMap, path::Path};

#[derive(Debug, Deserialize)]
struct DataDefinition {
    version: String,
    hash_map: HashMap<String, String>,
}

impl DataDefinition {
    fn load(fname: &Path) -> Self {
        load_yaml_file(&fname).unwrap()
    }
}

#[derive(Debug)]
pub struct DataMap {
    data_file_path: DataDefinition,
}

impl DataMap {
    /// Instantiate a DataMap with a
    /// specific data definition file (yaml)
    pub fn new(dfile: &Path) -> Self {
        Self {
            data_file_path: DataDefinition::load(dfile),
        }
    }
    /// Unpack data from a slice based on the
    /// data definition
    pub fn map_accounts_data(
        &self,
        rpc_client: &RpcClient,
        key: &Pubkey,
        commitment_config: CommitmentConfig,
    ) {
        println!("Processing account {:?}", key);
        let account = account_for_key(rpc_client, key, commitment_config).unwrap();
        println!("Made it here {:?}", account);
    }
}
