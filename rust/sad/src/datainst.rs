//! @brief Data instance

use std::{collections::BTreeMap, mem};

use crate::sad_errors::{SadAppError, SadTypeResult};

// Data mapping primary keys and supported data types
pub const DECL_TYPE_BORSH: &str = "borsh";
pub const DECL_TYPE_SERDE: &str = "serde";
pub const DECL_TYPE_KEY: &str = "type";
pub const DECL_TYPE_KEY_TYPE: &str = "key_type";
pub const DECL_TYPE_VALUE_TYPE: &str = "value_type";
pub const DECL_TYPE_BOOL_TYPE: &str = "bool";
pub const DECL_TYPE_U32_TYPE: &str = "u32";
pub const DECL_TYPE_U64_TYPE: &str = "u64";
pub const DECL_TYPE_STRING_TYPE: &str = "string";
pub const DECL_TYPE_ASSOCIATIVE_TYPE: &str = "associative";
pub const DECL_TYPE_ARRAY_TYPE: &str = "array";

pub trait SadType {
    fn decl_output_size(&self) -> usize;
    fn decl_input_size(&self) -> usize;
    fn decl_type(&self) -> &str;
    fn set_input_buffer(&mut self, input: &[u8]);
    fn input_buffer(&self) -> &[u8];
}

#[derive(Debug)]
struct DataGen<T> {
    type_value: T,
    type_input: Vec<u8>,
}

impl<T> DataGen<T> {
    fn new(val: T) -> Self {
        DataGen {
            type_value: val,
            type_input: Vec::<u8>::new(),
        }
    }
}

fn data_gen_from_string(tdesc: &String) -> SadTypeResult<Box<dyn SadType>> {
    if tdesc == DECL_TYPE_BOOL_TYPE {
        Ok(Box::new(DataGen::<bool>::new(true)))
    } else if tdesc == DECL_TYPE_U32_TYPE {
        Ok(Box::new(DataGen::<u32>::new(0u32)))
    } else if tdesc == DECL_TYPE_U64_TYPE {
        Ok(Box::new(DataGen::<u64>::new(0u64)))
    } else if tdesc == DECL_TYPE_STRING_TYPE {
        Ok(Box::new(DataGen::<String>::new(String::new())))
    } else if tdesc == DECL_TYPE_ASSOCIATIVE_TYPE {
        Ok(Box::new(DataGen::<BTreeMap<String, String>>::new(
            BTreeMap::<String, String>::new(),
        )))
    } else if tdesc == DECL_TYPE_ARRAY_TYPE {
        Ok(Box::new(DataGen::<Vec<String>>::new(Vec::<String>::new())))
    } else {
        Err(SadAppError::DataTypeUnknown {
            dtype: tdesc.to_string(),
        })
    }
}
#[derive(Default)]
pub struct DataInstance<'a> {
    raw_slice: &'a [u8],
    type_gens: Vec<Box<dyn SadType>>,
}

impl<'a> DataInstance<'a> {
    pub fn deconstruct(
        in_data: &'a [u8],
        data_def: &BTreeMap<String, BTreeMap<String, String>>,
    ) -> SadTypeResult<DataInstance<'a>> {
        let mut set_vec = Vec::<Box<dyn SadType>>::new();
        let mut buffer = in_data;
        for tl in data_def.keys() {
            let a_decl = data_def.get(tl).unwrap();
            let mut i_decl = data_gen_from_string(a_decl.get(DECL_TYPE_KEY).unwrap())?;
            i_decl.set_input_buffer(&buffer[..i_decl.decl_input_size()]);
            buffer = &buffer[i_decl.decl_input_size()..];
            println!(
                "Value {} advances buffer {} bytes",
                tl,
                i_decl.decl_input_size()
            );
            set_vec.push(i_decl);
        }
        Ok(Self {
            raw_slice: in_data,
            // src_array_ref: array_ref![in_data, 0, (data_size as usize)],
            type_gens: set_vec,
        })
    }
}

impl SadType for DataGen<bool> {
    fn decl_output_size(&self) -> usize {
        mem::size_of::<bool>()
    }

    fn decl_type(&self) -> &str {
        "bool"
    }

    fn decl_input_size(&self) -> usize {
        self.decl_output_size()
    }
    fn set_input_buffer(&mut self, input: &[u8]) {
        self.type_input = input.to_vec();
    }
    fn input_buffer(&self) -> &[u8] {
        &self.type_input
    }
}
impl SadType for DataGen<u32> {
    fn decl_output_size(&self) -> usize {
        mem::size_of::<u32>()
    }

    fn decl_type(&self) -> &str {
        "u32"
    }

    fn decl_input_size(&self) -> usize {
        self.decl_output_size()
    }
    fn set_input_buffer(&mut self, input: &[u8]) {
        self.type_input = input.to_vec();
    }
    fn input_buffer(&self) -> &[u8] {
        &self.type_input
    }
}
impl SadType for DataGen<u64> {
    fn decl_output_size(&self) -> usize {
        mem::size_of::<u64>()
    }

    fn decl_type(&self) -> &str {
        "u64"
    }

    fn decl_input_size(&self) -> usize {
        self.decl_output_size()
    }
    fn set_input_buffer(&mut self, input: &[u8]) {
        self.type_input = input.to_vec();
    }
    fn input_buffer(&self) -> &[u8] {
        &self.type_input
    }
}
impl SadType for DataGen<char> {
    fn decl_output_size(&self) -> usize {
        mem::size_of::<char>()
    }

    fn decl_type(&self) -> &str {
        "char"
    }

    fn decl_input_size(&self) -> usize {
        self.decl_output_size()
    }
    fn set_input_buffer(&mut self, input: &[u8]) {
        self.type_input = input.to_vec();
    }
    fn input_buffer(&self) -> &[u8] {
        &self.type_input
    }
}
impl SadType for DataGen<String> {
    fn decl_output_size(&self) -> usize {
        usize::MAX
    }

    fn decl_type(&self) -> &str {
        "string"
    }

    fn decl_input_size(&self) -> usize {
        todo!()
    }
    fn set_input_buffer(&mut self, input: &[u8]) {
        self.type_input = input.to_vec();
    }
    fn input_buffer(&self) -> &[u8] {
        &self.type_input
    }
}

impl SadType for DataGen<Vec<String>> {
    fn decl_output_size(&self) -> usize {
        usize::MAX
    }

    fn decl_type(&self) -> &str {
        "string"
    }

    fn decl_input_size(&self) -> usize {
        todo!()
    }
    fn set_input_buffer(&mut self, input: &[u8]) {
        self.type_input = input.to_vec();
    }
    fn input_buffer(&self) -> &[u8] {
        &self.type_input
    }
}

impl SadType for DataGen<BTreeMap<String, String>> {
    fn decl_output_size(&self) -> usize {
        usize::MAX
    }

    fn decl_type(&self) -> &str {
        "string"
    }

    fn decl_input_size(&self) -> usize {
        todo!()
    }
    fn set_input_buffer(&mut self, input: &[u8]) {
        self.type_input = input.to_vec();
    }
    fn input_buffer(&self) -> &[u8] {
        &self.type_input
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn types_test_pass() {
        let faux_buff = [1, 0, 0, 0, 1, 0, 0, 0];
        let mut faux_decl = BTreeMap::<String, BTreeMap<String, String>>::new();
        let mut faux_inner_decl = BTreeMap::<String, String>::new();
        faux_inner_decl.insert(DECL_TYPE_KEY.to_string(), DECL_TYPE_U32_TYPE.to_string());
        faux_decl.insert(String::from("Fake"), faux_inner_decl);
        let dinst = DataInstance::deconstruct(&faux_buff, &faux_decl).unwrap();

        for d in dinst.type_gens {
            println!(
                "Type {} and length {} with input buffer {:?}",
                d.decl_type(),
                d.decl_output_size(),
                d.input_buffer()
            )
        }
    }
}
