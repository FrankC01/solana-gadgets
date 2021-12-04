//! @brief sad outputs

use crate::{
    sadtypes::SadValue,
    solq::{DeserializationResult, ResultForKeyType},
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
    fn write_namedfields(&self, keyvalue: &Vec<SadValue>, level: &mut u32) {
        let key = match &keyvalue[0] {
            SadValue::String(s) => s,
            _ => unreachable!(),
        };
        print!("{:indent$}Key: {} ", "", key, indent = *level as usize);
        self.write_type(&keyvalue[1], level);
    }
    fn write_type(&self, value: &SadValue, level: &mut u32) {
        match value {
            SadValue::String(item) => {
                println!("{:indent$}String: {}", "", item, indent = *level as usize)
            }
            SadValue::Bool(item) => {
                println!("{:indent$}Boolean: {}", "", item, indent = *level as usize)
            }
            SadValue::U8(item) => {
                println!("{:indent$}U8: {}", "", item, indent = *level as usize)
            }
            SadValue::U16(item) => {
                println!("{:indent$}U16: {}", "", item, indent = *level as usize)
            }
            SadValue::U32(item) => {
                println!("{:indent$}U32: {}", "", item, indent = *level as usize)
            }
            SadValue::U64(item) => {
                println!("{:indent$}U64: {}", "", item, indent = *level as usize)
            }
            SadValue::U128(item) => {
                println!("{:indent$}U128: {}", "", item, indent = *level as usize)
            }
            SadValue::I8(item) => todo!(),
            SadValue::I16(item) => todo!(),
            SadValue::I32(item) => todo!(),
            SadValue::I64(item) => todo!(),
            SadValue::I128(item) => todo!(),
            SadValue::F32(item) => todo!(),
            SadValue::F64(item) => todo!(),
            SadValue::Vec(item) => todo!(),
            SadValue::Tuple(item) => todo!(),
            SadValue::HashMap(item) => {
                println!("{:indent$}HashMap:", "", indent = *level as usize);
                for nf in item {
                    *level += 1;
                    self.write_namedfields(nf, level);
                    *level -= 1
                }
            }
            SadValue::CStruct(item) => todo!(),
            SadValue::NamedField(item) => todo!(),
            _ => unreachable!(),
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
