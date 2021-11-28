use borsh::BorshDeserialize;
use lazy_static::*;
use std::{collections::HashMap, str::FromStr};
use strum::{EnumIter, EnumString, EnumVariantNames, VariantNames};

#[derive(Debug, EnumString, EnumIter, EnumVariantNames)]
pub enum SadValue {
    String(String),
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    F32(f32),
    F64(f64),
    Vec(Vec<SadValue>),
    Tuple(Vec<SadValue>),
    HashMap(Vec<Vec<SadValue>>),
    CStruct(Vec<SadValue>),
}

pub fn sad_value_from_sting(in_str: &str) -> SadValue {
    SadValue::from_str(in_str).unwrap()
}

pub fn is_sadvalue_type(in_str: &str) -> bool {
    match SadValue::VARIANTS.iter().position(|&r| r == in_str) {
        Some(_) => true,
        None => false,
    }
}
pub trait SadElement {
    fn deser(buf: &mut &[u8]) -> SadValue;
}

impl SadElement for String {
    fn deser(buf: &mut &[u8]) -> SadValue {
        let mlen = <u32>::try_from_slice(&buf[..4]).unwrap() as usize;
        if mlen > 0 {
            let fullsize = mlen + 4;
            let st = String::try_from_slice(&buf[..fullsize]).unwrap();
            *buf = &buf[fullsize..];
            SadValue::String(st)
        } else {
            *buf = &buf[4..];
            SadValue::String("".to_string())
        }
    }
}

impl SadElement for bool {
    fn deser(buf: &mut &[u8]) -> SadValue {
        let st = bool::try_from_slice(&buf[..1]).unwrap();
        *buf = &buf[1..];
        SadValue::Bool(st)
    }
}

impl SadElement for u8 {
    fn deser(buf: &mut &[u8]) -> SadValue {
        let st = u8::try_from_slice(&buf[..1]).unwrap();
        *buf = &buf[1..];
        SadValue::U8(st)
    }
}

impl SadElement for u16 {
    fn deser(buf: &mut &[u8]) -> SadValue {
        let st = u16::try_from_slice(&buf[..2]).unwrap();
        *buf = &buf[2..];
        SadValue::U16(st)
    }
}

impl SadElement for u32 {
    fn deser(buf: &mut &[u8]) -> SadValue {
        let st = u32::try_from_slice(&buf[..4]).unwrap();
        *buf = &buf[4..];
        SadValue::U32(st)
    }
}

impl SadElement for u64 {
    fn deser(buf: &mut &[u8]) -> SadValue {
        let st = u64::try_from_slice(&buf[..8]).unwrap();
        *buf = &buf[8..];
        SadValue::U64(st)
    }
}

impl SadElement for u128 {
    fn deser(buf: &mut &[u8]) -> SadValue {
        let st = u128::try_from_slice(&buf[..16]).unwrap();
        *buf = &buf[16..];
        SadValue::U128(st)
    }
}
impl SadElement for i8 {
    fn deser(buf: &mut &[u8]) -> SadValue {
        let st = i8::try_from_slice(&buf[..1]).unwrap();
        *buf = &buf[1..];
        SadValue::I8(st)
    }
}

impl SadElement for i16 {
    fn deser(buf: &mut &[u8]) -> SadValue {
        let st = i16::try_from_slice(&buf[..2]).unwrap();
        *buf = &buf[2..];
        SadValue::I16(st)
    }
}

impl SadElement for i32 {
    fn deser(buf: &mut &[u8]) -> SadValue {
        let st = i32::try_from_slice(&buf[..4]).unwrap();
        *buf = &buf[4..];
        SadValue::I32(st)
    }
}

impl SadElement for i64 {
    fn deser(buf: &mut &[u8]) -> SadValue {
        let st = i64::try_from_slice(&buf[..8]).unwrap();
        *buf = &buf[8..];
        SadValue::I64(st)
    }
}

impl SadElement for i128 {
    fn deser(buf: &mut &[u8]) -> SadValue {
        let st = i128::try_from_slice(&buf[..16]).unwrap();
        *buf = &buf[16..];
        SadValue::I128(st)
    }
}
impl SadElement for f32 {
    fn deser(buf: &mut &[u8]) -> SadValue {
        let st = f32::try_from_slice(buf).unwrap();
        *buf = &buf[4..];
        SadValue::F32(st)
    }
}

impl SadElement for f64 {
    fn deser(buf: &mut &[u8]) -> SadValue {
        let st = f64::try_from_slice(buf).unwrap();
        *buf = &buf[8..];
        SadValue::F64(st)
    }
}

lazy_static! {
    static ref DESER: HashMap<String, fn(&mut &[u8]) -> SadValue> = {
        let mut deser_table = HashMap::<String, fn(&mut &[u8]) -> SadValue>::new();
        deser_table.insert("String".to_string(), <String as SadElement>::deser);
        deser_table.insert("Bool".to_string(), <bool as SadElement>::deser);
        deser_table.insert("U8".to_string(), <u8 as SadElement>::deser);
        deser_table.insert("U16".to_string(), <u16 as SadElement>::deser);
        deser_table.insert("U32".to_string(), <u32 as SadElement>::deser);
        deser_table.insert("U64".to_string(), <u64 as SadElement>::deser);
        deser_table.insert("U128".to_string(), <u128 as SadElement>::deser);
        deser_table.insert("I8".to_string(), <i8 as SadElement>::deser);
        deser_table.insert("I16".to_string(), <i16 as SadElement>::deser);
        deser_table.insert("I32".to_string(), <i32 as SadElement>::deser);
        deser_table.insert("I64".to_string(), <i64 as SadElement>::deser);
        deser_table.insert("I128".to_string(), <i128 as SadElement>::deser);
        deser_table.insert("F32".to_string(), <f32 as SadElement>::deser);
        deser_table.insert("F64".to_string(), <f64 as SadElement>::deser);
        deser_table
    };
}

pub fn deser_value_for(key: &String, stream: &mut &[u8]) -> SadValue {
    let dfn = DESER.get(key).unwrap();
    dfn(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_types() {
        let vbool = vec![1u8];
        let result = DESER.get("Bool").unwrap();
        println!("{:?} {:?}", vbool, result(&mut vbool.as_slice()));
    }
}
