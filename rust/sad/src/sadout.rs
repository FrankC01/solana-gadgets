//! @brief sad outputs

use crate::{
    sadtypes::{from_scalar_value_for, is_sadtype_scalar, is_simple_compound, SadValue},
    solq::DeserializationResult,
};

/// Simple trait for
pub trait SadOutput: std::fmt::Debug {
    /// Clone of the inbound yaml sad 'type'
    fn deserialization_result(&self) -> &DeserializationResult;
    fn write(&self);
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
    fn write(&self) {
        let mut indent = 0;
        for blocks in self.deserialization_result().context_vec() {
            for sv in blocks.deserialize_list() {
                self.write_type(sv, &mut indent);
            }
        }
    }

    fn deserialization_result(&self) -> &DeserializationResult {
        &self.dresult
    }
}

/// Writes output to Excel file
#[derive(Debug)]
pub struct SadExcelOutput {
    dresult: DeserializationResult,
    file_name: String,
}

impl SadExcelOutput {
    pub fn new(data: DeserializationResult, out_file: &str) -> Self {
        Self {
            dresult: data,
            file_name: out_file.to_string(),
        }
    }
}

impl SadOutput for SadExcelOutput {
    fn write(&self) {
        println!(
            "Writing to EXCEL {} \n {:?}",
            self.file_name,
            self.deserialization_result()
        );
    }

    fn deserialization_result(&self) -> &DeserializationResult {
        &self.dresult
    }
}

/// Writes output to CSV file
#[derive(Debug)]
pub struct SadCsvOutput {
    dresult: DeserializationResult,
    file_name: String,
}

impl SadCsvOutput {
    pub fn new(data: DeserializationResult, out_file: &str) -> Self {
        Self {
            dresult: data,
            file_name: out_file.to_string(),
        }
    }
}

impl SadOutput for SadCsvOutput {
    fn write(&self) {
        println!(
            "Writing to CSV {} \n {:?}",
            self.file_name,
            self.deserialization_result()
        );
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
    use borsh::BorshSerialize;
    use gadgets_common::load_yaml_file;

    const INDEX_STRUCT_STRING_U32: usize = 7;
    #[derive(BorshSerialize)]
    struct OfStruct {
        name: String,
        age: u32,
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
