//! @brief Deserialization Support

use solana_sdk::pubkey::Pubkey;

use {
    borsh::BorshDeserialize,
    lazy_static::*,
    std::collections::HashMap,
    strum::{EnumIter, EnumProperty, EnumVariantNames, VariantNames},
};

#[derive(Clone, Debug, EnumIter, EnumVariantNames, EnumProperty)]
pub enum SadValue {
    #[strum(props(Type = "Scalar"))]
    String(String),
    #[strum(props(Type = "Scalar"))]
    Bool(bool),
    #[strum(props(Type = "Scalar"))]
    U8(u8),
    #[strum(props(Type = "Scalar"))]
    U16(u16),
    #[strum(props(Type = "Scalar"))]
    U32(u32),
    #[strum(props(Type = "Scalar"))]
    U64(u64),
    #[strum(props(Type = "Scalar"))]
    U128(u128),
    #[strum(props(Type = "Scalar"))]
    I8(i8),
    #[strum(props(Type = "Scalar"))]
    I16(i16),
    #[strum(props(Type = "Scalar"))]
    I32(i32),
    #[strum(props(Type = "Scalar"))]
    I64(i64),
    #[strum(props(Type = "Scalar"))]
    I128(i128),
    #[strum(props(Type = "Scalar"))]
    F32(f32),
    #[strum(props(Type = "Scalar"))]
    F64(f64),
    #[strum(props(Type = "Compound"))]
    Vec(Vec<SadValue>),
    #[strum(props(Type = "Compound"))]
    Tuple(Vec<SadValue>),
    #[strum(props(Type = "Nested Compound"))]
    HashMap(Vec<Vec<SadValue>>),
    #[strum(props(Type = "Compound"))]
    CStruct(Vec<SadValue>),
    #[strum(props(Type = "Compound"))]
    NamedField(Vec<SadValue>),
    #[strum(props(Type = "Scalar"))]
    PublicKey(Pubkey),
}

pub fn is_sadvalue_type(in_str: &str) -> bool {
    match SadValue::VARIANTS.iter().position(|&r| r == in_str) {
        Some(_) => true,
        None => false,
    }
}

pub fn is_sadtype_scalar(intype: &SadValue) -> bool {
    // First check if it's a valid type
    let s: &'static str = intype.get_str("Type").unwrap();
    if s == "Scalar" {
        true
    } else {
        false
    }
}

pub fn is_simple_compound(intype: &SadValue) -> bool {
    // First check if it's a valid type
    let s: &'static str = intype.get_str("Type").unwrap();
    if s == "Compound" {
        true
    } else {
        false
    }
}

pub fn from_scalar_value_for(intype: &SadValue) -> String {
    match intype {
        SadValue::String(item) => item.clone(),
        SadValue::Bool(item) => item.to_string(),
        SadValue::U8(item) => item.to_string(),
        SadValue::U16(item) => item.to_string(),
        SadValue::U32(item) => item.to_string(),
        SadValue::U64(item) => item.to_string(),
        SadValue::U128(item) => item.to_string(),
        SadValue::I8(item) => item.to_string(),
        SadValue::I16(item) => item.to_string(),
        SadValue::I32(item) => item.to_string(),
        SadValue::I64(item) => item.to_string(),
        SadValue::I128(item) => item.to_string(),
        SadValue::F32(item) => item.to_string(),
        SadValue::F64(item) => item.to_string(),
        SadValue::PublicKey(item) => item.to_string(),
        _ => unreachable!(),
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

impl SadElement for Pubkey {
    fn deser(buf: &mut &[u8]) -> SadValue {
        let st = Pubkey::try_from_slice(buf).unwrap();
        *buf = &buf[32..];
        SadValue::PublicKey(st)
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
        deser_table.insert("PublicKey".to_string(), <Pubkey as SadElement>::deser);
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
