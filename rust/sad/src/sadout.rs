//! @brief sad outputs

use std::fs::OpenOptions;

use crate::{
    desertree::Deseriaizer,
    errors::{SadAppErrorType, SadApplicationResult},
    sadtypes::{from_scalar_value_for, is_sadtype_scalar, is_simple_compound, SadValue},
    solq::DeserializationResult,
};

/// Retrieve the hashmap 'types' values
fn get_data_keyvalue(keyvalue: &Vec<SadValue>, collect: &mut Vec<String>) {
    let key = match &keyvalue[0] {
        SadValue::String(s) => s,
        _ => unreachable!(),
    };
    collect.push(key.to_string());
    get_data(&keyvalue[1], collect);
}

fn get_data(value: &SadValue, collect: &mut Vec<String>) {
    if is_sadtype_scalar(value) {
        collect.push(from_scalar_value_for(value))
    } else if is_simple_compound(value) {
        match value {
            SadValue::Vec(item) => {
                for vi in item {
                    get_data(vi, collect)
                }
            }
            SadValue::Tuple(item) => {
                for vi in item {
                    get_data(vi, collect)
                }
            }
            SadValue::CStruct(item) => {
                for nf in item {
                    get_data(nf, collect)
                }
            }
            SadValue::NamedField(item) => get_data_keyvalue(item, collect),
            _ => unreachable!(),
        }
    } else {
        match value {
            SadValue::HashMap(item) => {
                for nf in item {
                    get_data_keyvalue(nf, collect)
                }
            }
            _ => unreachable!(),
        }
    }
}
/// Simple trait for
pub trait SadOutput: std::fmt::Debug {
    /// Clone of the inbound yaml sad 'type'
    fn deserialization_result(&self) -> &DeserializationResult;
    fn write(&self) -> SadApplicationResult<()>;
}

/// Pretty prints output to sysout
#[derive(Debug)]
pub struct SadSysOutput {
    dresult: DeserializationResult,
}

impl SadSysOutput {
    pub fn new(data: DeserializationResult) -> Self {
        Self { dresult: data }
    }
    fn write_hashmap_keyvalue(&self, keyvalue: &Vec<SadValue>, level: &mut u32) {
        let key = match &keyvalue[0] {
            SadValue::String(s) => s,
            _ => unreachable!(),
        };
        print!("{:indent$}{}:", "", key, indent = *level as usize);
        self.write_type(&keyvalue[1], level);
    }

    fn write_type(&self, value: &SadValue, level: &mut u32) {
        if is_sadtype_scalar(value) {
            println!(
                "{:indent$}{}",
                "",
                from_scalar_value_for(value),
                indent = *level as usize
            )
        } else if is_simple_compound(value) {
            match value {
                SadValue::Vec(item) => {
                    println!("{:indent$}Vec [", "", indent = *level as usize);
                    *level += 1;
                    for vi in item {
                        self.write_type(vi, level)
                    }
                    *level -= 1;
                    println!("{:indent$}]", "", indent = *level as usize);
                }
                SadValue::Tuple(item) => {
                    println!("{:indent$}Tuple (", "", indent = *level as usize);
                    *level += 1;
                    for vi in item {
                        self.write_type(vi, level)
                    }
                    *level -= 1;
                    println!("{:indent$})", "", indent = *level as usize);
                }
                SadValue::CStruct(item) => {
                    println!("{:indent$}Structure {{", "", indent = *level as usize);
                    *level += 2;
                    for nf in item {
                        self.write_type(nf, level)
                    }
                    *level -= 2;
                    println!("{:indent$}}}", "", indent = *level as usize);
                }
                SadValue::NamedField(item) => self.write_hashmap_keyvalue(item, level),
                _ => unreachable!(),
            }
        } else {
            match value {
                SadValue::HashMap(item) => {
                    println!("{:indent$}HashMap {{", "", indent = *level as usize);
                    for nf in item {
                        *level += 1;
                        self.write_hashmap_keyvalue(nf, level);
                        *level -= 1;
                    }
                    println!("{:indent$}}}", "", indent = *level as usize);
                }
                _ => unreachable!(),
            }
        }
    }
}

impl SadOutput for SadSysOutput {
    fn write(&self) -> SadApplicationResult<()> {
        let mut indent = 0;
        for blocks in self.deserialization_result().context_vec() {
            for sv in blocks.deserialize_list() {
                self.write_type(sv, &mut indent);
            }
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
    ddecl: Deseriaizer,
    file_name: String,
}

impl SadJsonOutput {
    pub fn new(data: DeserializationResult, decl: Deseriaizer, out_file: &str) -> Self {
        Self {
            dresult: data,
            ddecl: decl,
            file_name: out_file.to_string(),
        }
    }
}

impl SadOutput for SadJsonOutput {
    fn write(&self) -> SadApplicationResult<()> {
        // let fpath = std::path::Path::new(&self.file_name);
        // // println!("{:?}", fpath.canonicalize()?);
        // let (fw, exists) = match fpath.exists() {
        //     true => (OpenOptions::new().append(true).open(fpath).unwrap(), true),
        //     false => (
        //         OpenOptions::new()
        //             .append(true)
        //             .create_new(true)
        //             .open(fpath)
        //             .unwrap(),
        //         false,
        //     ),
        // };
        // let mut wtr = csv::Writer::from_writer(fw);
        // let mut out_rows = Vec::<Vec<String>>::new();
        // let mut max_len = 0usize;
        // for c in self.deserialization_result().context_vec() {
        //     let mut out_row = Vec::<String>::new();
        //     out_row.push(c.pubkey().to_string());
        //     out_row.push(c.account().owner().to_string());
        //     for d in c.deserialize_list() {
        //         get_data(d, &mut out_row)
        //     }
        //     // Get maximum size
        //     if out_row.len() > max_len {
        //         max_len = out_row.len()
        //     }

        //     out_rows.push(out_row)
        // }
        // // Set equal row lengths and write record
        // for r in out_rows.iter_mut() {
        //     let diff = max_len - r.len();
        //     if diff != 0 {
        //         for _ in 0..diff {
        //             r.push(String::new())
        //         }
        //     }
        //     if r.len() != max_len {
        //         return Err(SadAppErrorType::InconsistentRowLength(max_len, r.len()));
        //     }
        //     wtr.write_record(r).unwrap();
        //     wtr.flush().unwrap();
        // }
        Ok(())
    }

    fn deserialization_result(&self) -> &DeserializationResult {
        &self.dresult
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        desertree::Deseriaizer,
        sadtypes::{from_scalar_value_for, is_sadtype_scalar, is_simple_compound},
    };

    use super::*;
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
        println!("{:?}", desc.schema());
        let deserialize_vector = desc.deser(&mut pacv.as_slice());
        println!("{:?}", deserialize_vector.unwrap());
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
