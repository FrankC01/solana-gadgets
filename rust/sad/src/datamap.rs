//! @brief Data map

use crate::{
    datainst::*,
    sad_errors::{SadAppError, SadBaseResult, SadTypeResult},
};
use gadgets_common::load_yaml_file;
use lazy_static::lazy_static;
use serde::Deserialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{account::ReadableAccount, commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

// Supported deserialization set of options
lazy_static! {
    pub static ref DECL_DESERIAL_SET: BTreeSet<String> = {
        let mut hs = BTreeSet::<String>::new();
        hs.insert(DECL_TYPE_BORSH.to_string());
        hs.insert(DECL_TYPE_SERDE.to_string());
        hs
    };
}

// Set of type keys
lazy_static! {
    pub static ref DECL_TYPE_SET: BTreeSet<String> = {
        let mut hs = BTreeSet::<String>::new();
        hs.insert(DECL_TYPE_KEY.to_string());
        hs.insert(DECL_TYPE_KEY_TYPE.to_string());
        hs.insert(DECL_TYPE_VALUE_TYPE.to_string());
        hs
    };
}

// Set of types
lazy_static! {
    pub static ref TYPE_SET: BTreeSet<String> = {
        let mut hs = BTreeSet::<String>::new();
        hs.insert(DECL_TYPE_BOOL_TYPE.to_string());
        hs.insert(DECL_TYPE_U32_TYPE.to_string());
        hs.insert(DECL_TYPE_U64_TYPE.to_string());
        hs.insert(DECL_TYPE_STRING_TYPE.to_string());
        hs.insert(DECL_TYPE_ASSOCIATIVE_TYPE.to_string());
        hs.insert(DECL_TYPE_ARRAY_TYPE.to_string());
        hs
    };
}

/// DataDefinition describes the `data` for a given account
#[derive(Debug, Deserialize)]
pub struct DataDefinition {
    version: String,
    deserializer: String,
    total_data_size: u32,
    data_mapping: BTreeMap<String, BTreeMap<String, String>>,
}

impl DataDefinition {
    /// Simple validator
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
                        let _d = DataInstance::deconstruct(
                            &account.data,
                            &self.data_definition.data_mapping,
                        );
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
        let y = DataDefinition::load(&path_from_str("rust/yaml_samps/hbclisamp.yml")).unwrap();
        assert_eq!(y.version, String::from("0.1.0"));
    }

    #[test]
    fn load_datadefinition_fail() {
        let y = DataDefinition::load(&path_from_str("rust/yaml_samps/hbclisamp_bad.yml"));
        assert!(y.is_err());
        match y {
            Ok(_) => panic!("Should never have gotten here"),
            Err(w) => eprint!("{}", w),
        }
    }
}
