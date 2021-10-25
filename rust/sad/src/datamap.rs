//! @brief Data map

use crate::sad_errors::{SadAppError, SadBaseResult, SadTypeResult};
use gadgets_common::load_yaml_file;

use lazy_static::lazy_static;
use serde::Deserialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

// Data mapping primary keys and supported data types
lazy_static! {
    static ref DECL_TYPE_BORSH: String = String::from("borsh");
    static ref DECL_TYPE_SERDE: String = String::from("serde");
    static ref DECL_TYPE_KEY: String = String::from("type");
    static ref DECL_TYPE_KEY_TYPE: String = String::from("key_type");
    static ref DECL_TYPE_VALUE_TYPE: String = String::from("value_type");
    static ref DECL_TYPE_BOOL_TYPE: String = String::from("bool");
    static ref DECL_TYPE_U32_TYPE: String = String::from("u32");
    static ref DECL_TYPE_U64_TYPE: String = String::from("u64");
    static ref DECL_TYPE_STRING_TYPE: String = String::from("string");
    static ref DECL_TYPE_ASSOCIATIVE_TYPE: String = String::from("associative");
}

// Supported deserialization set of options
lazy_static! {
    static ref DECL_DESERIAL_SET: HashSet<String> = {
        let mut hs = HashSet::<String>::new();
        hs.insert(DECL_TYPE_BORSH.to_string());
        hs.insert(DECL_TYPE_SERDE.to_string());
        hs
    };
}

// Set of type keys
lazy_static! {
    static ref DECL_TYPE_SET: HashSet<String> = {
        let mut hs = HashSet::<String>::new();
        hs.insert(DECL_TYPE_KEY.to_string());
        hs.insert(DECL_TYPE_KEY_TYPE.to_string());
        hs.insert(DECL_TYPE_VALUE_TYPE.to_string());
        hs
    };
}

// Set of types
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

/// DataDefinition describes the `data` for a given account
#[derive(Debug, Deserialize)]
struct DataDefinition {
    version: String,
    deserializer: String,
    total_data_size: u32,
    data_mapping: HashMap<String, HashMap<String, String>>,
}

impl DataDefinition {
    /// Validates various keys and values in a yaml loaded
    /// DataDefinition file
    fn check_types(&self) -> SadBaseResult {
        if !DECL_DESERIAL_SET.contains(&self.deserializer) {
            return Err(SadAppError::DataMappingUnknownDeserializer {
                value: self.deserializer.clone(),
            });
        }
        for hlmap in self.data_mapping.keys() {
            let mymap = self.data_mapping.get(hlmap).unwrap();
            if !mymap.contains_key(&*DECL_TYPE_KEY) {
                return Err(SadAppError::DataMappingMissingTypeError { key: hlmap.clone() });
            } else if !TYPE_SET.contains(mymap.get(&*DECL_TYPE_KEY).unwrap()) {
                return Err(SadAppError::DataMappingError {
                    key: hlmap.clone(),
                    value: mymap.get(&*DECL_TYPE_KEY).unwrap().clone(),
                });
            }
            if mymap.get(&*DECL_TYPE_KEY).unwrap() == &*DECL_TYPE_ASSOCIATIVE_TYPE {
                if mymap.len() < 3 {
                    return Err(SadAppError::DataMappingCountError {
                        length: mymap.len(),
                    });
                }
                for (key, value) in mymap.iter() {
                    if !DECL_TYPE_SET.contains(key) || !TYPE_SET.contains(value) {
                        return Err(SadAppError::DataMappingElementError {
                            element: hlmap.clone(),
                            key: key.clone(),
                            value: value.clone(),
                        });
                    }
                }
            }
        }
        Ok(())
    }
    fn load(fname: &Path) -> SadTypeResult<DataDefinition> {
        match load_yaml_file(fname) {
            Ok(dd) => {
                let myd: DataDefinition = dd;
                myd.check_types()?;
                Ok(myd)
            }
            Err(e_) => Err(SadAppError::IoError(e_)),
        }
    }
}

#[derive(Debug)]
pub struct DataMap {
    data_definition: DataDefinition,
}

impl DataMap {
    /// Instantiate a DataMap with a
    /// specific data definition file (yaml)
    pub fn new(dfile: &Path) -> SadTypeResult<DataMap> {
        match DataDefinition::load(dfile) {
            Ok(x_) => Ok(Self {
                data_definition: x_,
            }),
            Err(e_) => Err(e_),
        }
    }
    /// Unpack data from a slice based on the
    /// data definition
    pub fn map_accounts_data(
        &self,
        rpc_client: &RpcClient,
        key: &Pubkey,
        commitment_config: CommitmentConfig,
        show_raw: bool,
    ) -> SadBaseResult {
        match rpc_client.get_account_with_commitment(key, commitment_config) {
            Ok(a_) => match a_.value {
                Some(account) => {
                    if account.executable {
                        Err(SadAppError::AccountIsExecutable(*key))
                    } else if account.data.is_empty() {
                        Err(SadAppError::AccountHasNoData(*key))
                    } else {
                        println!("\nAccount #: {}", key);
                        println!("Remaining Lamports: {}", account.lamports);
                        println!("Data size (bytes): {}", account.data.len());
                        if show_raw {
                            println!("Raw data: {:?}", account.data);
                        }
                        // Decompose the data
                        Ok(())
                    }
                }
                None => Err(SadAppError::NoAccount(*key)),
            },
            Err(e_) => Err(SadAppError::ConnectionError(e_)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env::current_dir, path::PathBuf};

    fn path_from_str(in_str: &'static str) -> PathBuf {
        current_dir().unwrap().parent().unwrap().join(in_str)
    }
    #[test]
    fn load_datadefinition_pass() {
        let y = DataDefinition::load(&path_from_str("yaml_samps/hbclisamp.yml")).unwrap();
        assert_eq!(y.version, String::from("0.1.0"));
        assert_eq!(y.deserializer, String::from("borsh"));
        let initialized = String::from("initialized");
        let btree_len = String::from("btree_len");
        let btree = String::from("btree");
        assert!(y.data_mapping.contains_key(&initialized));
        assert_eq!(
            y.data_mapping
                .get(&initialized)
                .unwrap()
                .get(&*DECL_TYPE_KEY)
                .unwrap(),
            &*DECL_TYPE_BOOL_TYPE
        );
        assert!(y.data_mapping.contains_key(&btree_len));
        assert_eq!(
            y.data_mapping
                .get(&btree_len)
                .unwrap()
                .get(&*DECL_TYPE_KEY)
                .unwrap(),
            &*DECL_TYPE_U32_TYPE
        );
        assert!(y.data_mapping.contains_key(&btree));
        let btree_type = y.data_mapping.get(&btree).unwrap();
        assert_eq!(
            btree_type.get(&*DECL_TYPE_KEY).unwrap(),
            &*DECL_TYPE_ASSOCIATIVE_TYPE
        );
        assert_eq!(
            btree_type.get(&*DECL_TYPE_KEY_TYPE).unwrap(),
            &*DECL_TYPE_STRING_TYPE
        );
        assert_eq!(
            btree_type.get(&*DECL_TYPE_VALUE_TYPE).unwrap(),
            &*DECL_TYPE_STRING_TYPE
        );
    }

    #[test]
    fn load_datadefinition_fail() {
        let y = DataDefinition::load(&path_from_str("yaml_samps/hbclisamp_bad.yml"));
        assert!(y.is_err());
        match y {
            Ok(_) => panic!("Should never have gotten here"),
            Err(w) => eprint!("{}", w),
        }
    }
}
