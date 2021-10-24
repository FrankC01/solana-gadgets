/// solana-gadgets common resuable modules
/// Includes
///
use serde_yaml::{self, from_reader};
use std::{fs::File, io, path::Path};

/// Loads a yaml_file
pub fn load_yaml_file<T, P>(yaml_file: P) -> Result<T, io::Error>
where
    T: serde::de::DeserializeOwned,
    P: AsRef<Path>,
{
    let file = File::open(yaml_file)?;
    let result = from_reader(file)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("{:?}", err)))?;
    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;
    // use crate::load_yaml_file;
    use ::{
        serde::{Deserialize, Serialize},
        std::{collections::HashMap, path::Path},
    };

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct Simple {
        version: String,
        data_mapping: HashMap<String, String>,
    }

    impl Simple {
        fn load(fname: &str) -> Result<Self, io::Error> {
            load_yaml_file(&Path::new(fname))
        }
    }

    #[test]
    fn load_yaml_file_pass() {
        let y = Simple::load("../yaml_samps/test.yml").unwrap();
        assert_eq!(y.version, String::from("0.1.0"));
    }
    #[test]
    fn load_yaml_file_fail() {
        let y = Simple::load("../yaml_samps/test_fail.yml");
        assert!(y.is_err());
    }
}
