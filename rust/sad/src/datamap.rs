//! @brief Data map

use gadgets_common::load_yaml_file;
use gadgets_common::sol_txns::account_for_key;
use lazy_static::lazy_static;
use serde::Deserialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::{
    collections::{HashMap, HashSet},
    io,
    path::Path,
};

lazy_static! {
    static ref DECL_TYPE_KEY: String = String::from("type");
    static ref DECL_TYPE_KEY_TYPE: String = String::from("key_type");
    static ref DECL_TYPE_VALUE_TYPE: String = String::from("value_type");
    static ref DECL_TYPE_BOOL_TYPE: String = String::from("bool");
    static ref DECL_TYPE_U32_TYPE: String = String::from("u32");
    static ref DECL_TYPE_U64_TYPE: String = String::from("u64");
    static ref DECL_TYPE_STRING_TYPE: String = String::from("string");
    static ref DECL_TYPE_ASSOCIATIVE_TYPE: String = String::from("associative");
}

lazy_static! {
    static ref DECL_TYPE_SET: HashSet<String> = {
        let mut hs = HashSet::<String>::new();
        hs.insert(DECL_TYPE_KEY.to_string());
        hs.insert(DECL_TYPE_KEY_TYPE.to_string());
        hs.insert(DECL_TYPE_VALUE_TYPE.to_string());
        hs
    };
}
lazy_static! {
    static ref TYPE_SET: HashSet<String> = {
        let mut hs = HashSet::<String>::new();
        hs.insert(DECL_TYPE_BOOL_TYPE.to_string());
        hs.insert(DECL_TYPE_U32_TYPE.to_string());
        hs.insert(DECL_TYPE_U64_TYPE.to_string());
        hs.insert(DECL_TYPE_STRING_TYPE.to_string());
        hs.insert(DECL_TYPE_ASSOCIATIVE_TYPE.to_string());
        hs
    };
}
#[derive(Debug, Deserialize)]
struct DataDefinition {
    version: String,
    deserializer: String,
    total_data_size: u32,
    data_mapping: HashMap<String, HashMap<String, String>>,
}

impl DataDefinition {
    fn load(fname: &Path) -> Result<Self, io::Error> {
        load_yaml_file(fname)
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
            data_file_path: DataDefinition::load(dfile).unwrap(),
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
        let account = account_for_key(rpc_client, key, commitment_config).unwrap();
        println!("\nAccount #:{}", key);
        println!("Remaining Lamports: {}", account.lamports);
        println!("Data size (bytes): {}", account.data.len());
        println!("Raw data: {:?}", account.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::current_dir;

    #[test]
    fn load_datadefinition() {
        let fpath = current_dir()
            .unwrap()
            .parent()
            .unwrap()
            .join("yaml_samps/hbclisamp.yml");
        let y = DataDefinition::load(&fpath).unwrap();
        assert_eq!(y.version, String::from("0.1.0"));
        assert_eq!(y.deserializer, String::from("borsh"));
        let initialized = String::from("initialized");
        let btree_len = String::from("btree_len");
        let btree = String::from("btree");
        let dtype = String::from("type");
        let ktype = String::from("key_type");
        let vtype = String::from("value_type");
        assert!(y.data_mapping.contains_key(&initialized));
        assert_eq!(
            y.data_mapping
                .get(&initialized)
                .unwrap()
                .get(&dtype)
                .unwrap(),
            &String::from("bool")
        );
        assert!(y.data_mapping.contains_key(&btree_len));
        assert_eq!(
            y.data_mapping.get(&btree_len).unwrap().get(&dtype).unwrap(),
            &String::from("u32")
        );
        assert!(y.data_mapping.contains_key(&btree));
        let btree_type = y.data_mapping.get(&btree).unwrap();
        assert_eq!(
            btree_type.get(&dtype).unwrap(),
            &String::from("associative")
        );
        assert_eq!(btree_type.get(&ktype).unwrap(), &String::from("string"));
        assert_eq!(btree_type.get(&vtype).unwrap(), &String::from("string"));
    }
}
