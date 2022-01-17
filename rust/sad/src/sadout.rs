//! @brief sad outputs

use crate::{desertree::Deseriaizer, errors::SadApplicationResult, solq::DeserializationResult};
use serde_json::{from_str, json, to_string_pretty};
use std::{
    fs::{read_to_string, File},
    io::Write,
};

/// Simple trait for
pub trait SadOutput: std::fmt::Debug {
    /// Clone of the inbound yaml sad 'type'
    fn deserialization_result(&self) -> &DeserializationResult;
    fn write(&self) -> SadApplicationResult<()>;
}

/// Pretty prints output to sysout
#[derive(Debug)]
pub struct SadSysOutput {
    deser: Deseriaizer,
    dresult: DeserializationResult,
}

impl SadSysOutput {
    pub fn new(data: DeserializationResult, ddecl: Deseriaizer) -> Self {
        Self {
            deser: ddecl,
            dresult: data,
        }
    }
}

impl SadOutput for SadSysOutput {
    fn write(&self) -> SadApplicationResult<()> {
        for blocks in self.deserialization_result().context_vec() {
            println!(
                "{}",
                to_string_pretty(&self.deser.to_json(blocks.deserialize_list())).unwrap()
            );
        }
        Ok(())
    }

    fn deserialization_result(&self) -> &DeserializationResult {
        &self.dresult
    }
}

/// Writes output to CSV file
#[derive(Debug)]
pub struct SadJsonOutput {
    dresult: DeserializationResult,
    deser: Deseriaizer,
    file_name: String,
}

impl SadJsonOutput {
    pub fn new(data: DeserializationResult, decl: Deseriaizer, out_file: &str) -> Self {
        Self {
            dresult: data,
            deser: decl,
            file_name: out_file.to_string(),
        }
    }
}

impl SadOutput for SadJsonOutput {
    fn write(&self) -> SadApplicationResult<()> {
        let fpath = std::path::Path::new(&self.file_name);
        let mut json_vector = if fpath.exists() {
            let data = read_to_string(fpath).expect("Unable to read file");
            from_str(&data).expect("Unable to parse")
        } else {
            json!([])
        };
        for c in self.deserialization_result().context_vec() {
            let mut jmap = json!({});
            let jmap_raw = jmap.as_object_mut().unwrap();
            jmap_raw.insert("account_key".to_string(), json!(c.pubkey().to_string()));
            jmap_raw.insert(
                "account_program_key".to_string(),
                json!(c.account().owner.to_string()),
            );
            jmap_raw.insert("data".to_string(), self.deser.to_json(c.deserialize_list()));
            json_vector.as_array_mut().unwrap().push(jmap);
        }
        let ppjson = to_string_pretty(&json_vector).unwrap();
        let mut file = File::create(fpath).unwrap();
        // let mut writer = BufWriter::new(file);
        // serde_json::to_writer(&mut writer, &json_vector).unwrap();
        file.write(ppjson.as_bytes()).unwrap();
        Ok(())
    }

    fn deserialization_result(&self) -> &DeserializationResult {
        &self.dresult
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::sadtypes::{from_scalar_value_for, is_sadtype_scalar, is_simple_compound, SadValue};
    use base64::decode;
    use borsh::BorshSerialize;
    use gadgets_common::load_yaml_file;
    use yaml_rust::Yaml;

    const INDEX_STRUCT_STRING_U32: usize = 7;
    #[derive(BorshSerialize)]
    struct OfStruct {
        name: String,
        age: u32,
    }
    /// vscode changes cwd depending on running test or debugging test
    fn get_sample_yaml() -> Vec<Yaml> {
        if std::env::current_dir().unwrap().ends_with("sad") {
            load_yaml_file(
                "../../samples/yamldecls/SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.yml",
            )
            .unwrap()
        } else {
            load_yaml_file("../samples/yamldecls/SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.yml")
                .unwrap()
        }
    }
    #[test]
    fn test_deserialization_pass() {
        let pacc = "AU8AAAADAAAABQAAAEhhcHB5CQAAAE5ldyBZZWFyIQYAAABuZXdLZXkLAAAAQSBuZXcgdmFsdWUGAAAAdHMga2V5DgAAAHRzIGZpcnN0IHZhbHVldCB2YWx1ZTIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==";
        let pacv = decode(pacc).unwrap();
        let result = get_sample_yaml();
        let desc = Deseriaizer::new(&result[0]);
        let deserialize_vector = desc.deser(&mut pacv.as_slice()).unwrap();
        println!(
            "{}",
            to_string_pretty(&desc.to_json(&deserialize_vector)).unwrap()
        );
    }

    fn write_hashmap_keyvalue(keyvalue: &Vec<SadValue>, level: &mut u32) {
        let key = match &keyvalue[0] {
            SadValue::String(s) => s,
            _ => unreachable!(),
        };
        print!("{:indent$}{}: ", "", key, indent = *level as usize);
        write_type(&keyvalue[1], level);
    }

    fn write_type(value: &SadValue, level: &mut u32) {
        if is_sadtype_scalar(value) {
            println!(
                "{:indent$} {}",
                "",
                from_scalar_value_for(value),
                indent = *level as usize
            )
        } else if is_simple_compound(value) {
            match value {
                SadValue::Vec(item) => {
                    println!("{:indent$}Vec:", "", indent = *level as usize);
                    *level += 1;
                    for vi in item {
                        write_type(vi, level)
                    }
                    *level -= 1
                }
                SadValue::Tuple(item) => {
                    println!("{:indent$}Tuple:", "", indent = *level as usize);
                    *level += 1;
                    for vi in item {
                        write_type(vi, level)
                    }
                    *level -= 1
                }
                SadValue::CStruct(item) => {
                    println!("{:indent$}Structure {{", "", indent = *level as usize);
                    *level += 2;
                    for nf in item {
                        write_type(nf, level)
                    }
                    *level -= 2;
                    println!("{:indent$}}}", "", indent = *level as usize);
                }
                SadValue::NamedField(item) => write_hashmap_keyvalue(item, level),
                _ => unreachable!(),
            }
        } else {
            match value {
                SadValue::HashMap(item) => {
                    println!("{:indent$}HashMap {{", "", indent = *level as usize);
                    for nf in item {
                        *level += 1;
                        write_hashmap_keyvalue(nf, level);
                        *level -= 1;
                        println!("{:indent$}}}", "", indent = *level as usize);
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    fn write(svals: &Vec<SadValue>) {
        let mut indent = 0;
        for sv in svals {
            write_type(sv, &mut indent);
        }
    }

    #[test]
    fn test_cstruct_out() {
        let mhmap = OfStruct {
            name: "Frank".to_string(),
            age: 64,
        };
        // println!("{:?}", std::env::current_dir());
        let result = if std::env::current_dir().unwrap().ends_with("sad") {
            load_yaml_file("../yaml_samps/runner.yml").unwrap()
        } else {
            load_yaml_file("./yaml_samps/runner.yml").unwrap()
        };

        let desc = Deseriaizer::new(&result[INDEX_STRUCT_STRING_U32]);
        let data = mhmap.try_to_vec().unwrap();
        write(&desc.deser(&mut data.as_slice()).unwrap());
    }
}
