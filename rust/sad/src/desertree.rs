//! @brief Heart of deserialization
//! Constructs for instantiating parse tree with constructs
//! from YAML declaration and then walking said tree to deserialize
//! input vector of bytes that come from a program owned account data

use solana_sdk::pubkey::Pubkey;

use {
    crate::{
        errors::{SadTreeError, SadTreeResult},
        sadtypes::{deser_value_for, is_sadvalue_type, SadValue},
    },
    borsh::BorshDeserialize,
    downcast_rs::{impl_downcast, Downcast},
    lazy_static::*,
    std::collections::HashMap,
    yaml_rust::{yaml::Yaml, YamlLoader},
};
/// Simple Node for tree membership
trait Node: std::fmt::Debug + Downcast {
    /// Clone of the inbound yaml sad 'type'
    fn decl_type(&self) -> &String;
    fn deser(&self, data: &mut &[u8], collection: &mut Vec<SadValue>);
}
impl_downcast!(Node);

/// Simple branch for tree membership
trait NodeWithChildren: Node {
    fn children(&self) -> &Vec<Box<dyn Node>>;
}
impl_downcast!(NodeWithChildren);

const SAD_YAML_TYPE: &str = "type";
const SAD_YAML_NAME: &str = "name";
const SAD_YAML_DESCRIPTOR: &str = "descriptor";
const SAD_YAML_SIZE_TYPE: &str = "size_type";
const SAD_YAML_CONTAINS: &str = "contains";
const SAD_YAML_FIELDS: &str = "fields";
const SAD_NAMED_FIELD: &str = "NamedField";

// Jump table for generalizing parse construction
lazy_static! {
    static ref SAD_TYPE_JSON: Vec<Yaml> =
        YamlLoader::load_from_str(&format!("{}", SAD_YAML_TYPE)).unwrap();
    static ref JUMP_TABLE: HashMap<String, fn(&Yaml) -> Result<Box<dyn Node>, SadTreeError>> = {
        let mut jump_table =
            HashMap::<String, fn(&Yaml) -> Result<Box<dyn Node>, SadTreeError>>::new();
        jump_table.insert("length_prefix".to_string(), SadLengthPrefix::from_yaml);
        jump_table.insert("HashMap".to_string(), SadHashMap::from_yaml);
        jump_table.insert("Vec".to_string(), SadVector::from_yaml);
        jump_table.insert("Tuple".to_string(), SadTuple::from_yaml);
        jump_table.insert("CStruct".to_string(), SadStructure::from_yaml);
        jump_table.insert("NamedField".to_string(), SadNamedField::from_yaml);
        jump_table.insert("PublicKey".to_string(), SadPublicKey::from_yaml);
        jump_table.insert("other".to_string(), SadLeaf::from_yaml);
        jump_table
    };
    static ref SAD_JUMP_OTHER: fn(&Yaml) -> Result<Box<dyn Node>, SadTreeError> =
        *JUMP_TABLE.get("other").unwrap();
    static ref SAD_PUBKEY_CHILD: Vec<Yaml> =
        YamlLoader::load_from_str(&format!("{}", "type: Vec\ncontains: \n    - type: U32"))
            .unwrap();
}

/// Implements Node for low level scalar types
#[derive(Debug)]
pub struct SadLeaf {
    sad_value_type: String,
}

impl SadLeaf {
    fn from_yaml(in_yaml: &Yaml) -> SadTreeResult<Box<dyn Node>> {
        let in_str = in_yaml[SAD_YAML_TYPE].as_str().unwrap();
        if is_sadvalue_type(in_str) {
            Ok(Box::new(SadLeaf {
                sad_value_type: String::from(in_str),
            }))
        } else {
            Err(SadTreeError::UnknownType(String::from(in_str)))
        }
    }
}

impl Node for SadLeaf {
    fn decl_type(&self) -> &String {
        &self.sad_value_type
    }

    fn deser(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        collection.push(deser_value_for(self.decl_type(), data));
    }
}

#[derive(Debug)]
pub struct SadPublicKey {
    sad_value_type: String,
    children: Vec<Box<dyn Node>>,
}

impl SadPublicKey {
    fn from_yaml(in_yaml: &Yaml) -> SadTreeResult<Box<dyn Node>> {
        let in_str = in_yaml[SAD_YAML_TYPE].as_str().unwrap();
        if is_sadvalue_type(in_str) {
            let mut array = Vec::<Box<dyn Node>>::new();
            array.push(parse(&SAD_PUBKEY_CHILD[0])?);
            Ok(Box::new(SadPublicKey {
                sad_value_type: String::from(in_str),
                children: array,
            }))
        } else {
            Err(SadTreeError::UnknownType(String::from(in_str)))
        }
    }
}

impl Node for SadPublicKey {
    fn decl_type(&self) -> &String {
        &self.sad_value_type
    }

    fn deser(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        // let mut coll = Vec::<SadValue>::new();
        let pk = Pubkey::try_from_slice(data).unwrap();
        // coll.push(pk);
        // for c in &self.children {
        //     c.deser(data, &mut coll)
        // }
        collection.push(SadValue::PublicKey(pk));
    }
}
impl NodeWithChildren for SadPublicKey {
    fn children(&self) -> &Vec<Box<dyn Node>> {
        &self.children
    }
}

/// Implements NodeWithChildren for SadStructure Named Fields
#[derive(Debug)]
pub struct SadNamedField {
    sad_field_name: String,
    sad_value_type: String,
    children: Vec<Box<dyn Node>>,
}

impl SadNamedField {
    fn from_yaml(in_yaml: &Yaml) -> SadTreeResult<Box<dyn Node>> {
        let desc = &in_yaml[SAD_YAML_DESCRIPTOR];
        let in_name = desc[SAD_YAML_NAME].as_str().unwrap();
        let mut array = Vec::<Box<dyn Node>>::new();
        let contains = &desc["contains"];
        array.push(parse(contains)?);
        Ok(Box::new(SadNamedField {
            sad_field_name: String::from(in_name),
            sad_value_type: String::from(SAD_NAMED_FIELD),
            children: array,
        }))
    }

    fn name(&self) -> &String {
        &self.sad_field_name
    }
}

impl Node for SadNamedField {
    fn decl_type(&self) -> &String {
        &self.sad_value_type
    }

    fn deser(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        let mut coll = Vec::<SadValue>::new();
        coll.push(SadValue::String(self.name().clone()));
        for c in &self.children {
            c.deser(data, &mut coll)
        }
        collection.push(SadValue::NamedField(coll));
    }
}
impl NodeWithChildren for SadNamedField {
    fn children(&self) -> &Vec<Box<dyn Node>> {
        &self.children
    }
}

/// Implements NodeWithChildren for length prefixed children
#[derive(Debug)]
pub struct SadLengthPrefix {
    sad_value_type: String,
    sad_length_type: String,
    children: Vec<Box<dyn Node>>,
}
impl SadLengthPrefix {
    fn from_yaml(in_yaml: &Yaml) -> SadTreeResult<Box<dyn Node>> {
        let in_str = in_yaml[SAD_YAML_SIZE_TYPE].as_str().unwrap();
        if !is_sadvalue_type(in_str) {
            return Err(SadTreeError::UnknownType(String::from(in_str)));
        }
        let in_type_str = in_yaml[SAD_YAML_TYPE].as_str().unwrap();
        let mut array = Vec::<Box<dyn Node>>::new();
        let contains = &in_yaml[SAD_YAML_CONTAINS];
        match contains {
            Yaml::Array(lst) => {
                array.push(parse(&lst[0]).unwrap());
                Ok(Box::new(SadLengthPrefix {
                    sad_value_type: String::from(in_type_str),
                    sad_length_type: String::from(in_str),
                    children: array,
                }))
            }
            Yaml::Hash(_map) => {
                array.push(parse(contains).unwrap());
                Ok(Box::new(SadLengthPrefix {
                    sad_value_type: String::from(in_type_str),
                    sad_length_type: String::from(in_str),
                    children: array,
                }))
            }
            _ => Err(SadTreeError::ExpectedHashMapOrArray),
        }
    }

    pub fn length_type(&self) -> &String {
        &self.sad_length_type
    }
}
impl Node for SadLengthPrefix {
    fn decl_type(&self) -> &String {
        &self.sad_value_type
    }

    fn deser(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        let overall = u32::try_from_slice(&data[0..4]).unwrap();
        *data = &data[4..];
        collection.push(SadValue::U32(overall));
        for c in &self.children {
            c.deser(data, collection)
        }
    }
}
impl NodeWithChildren for SadLengthPrefix {
    fn children(&self) -> &Vec<Box<dyn Node>> {
        &self.children
    }
}

/// Implements NodeWithChildren for HashMap
#[derive(Debug)]
pub struct SadHashMap {
    sad_value_type: String,
    children: Vec<Box<dyn Node>>,
}

impl SadHashMap {
    fn from_yaml(in_yaml: &Yaml) -> SadTreeResult<Box<dyn Node>> {
        let in_str = in_yaml[SAD_YAML_TYPE].as_str().unwrap();
        let mut array = Vec::<Box<dyn Node>>::new();
        let fields = &in_yaml[SAD_YAML_FIELDS];
        match fields {
            Yaml::Array(lst) => {
                for hl in lst {
                    array.push(parse(hl).unwrap())
                }
                Ok(Box::new(SadHashMap {
                    sad_value_type: String::from(in_str),
                    children: array,
                }))
            }
            _ => Err(SadTreeError::ExpectedHashMapFields),
        }
    }
}
impl Node for SadHashMap {
    fn decl_type(&self) -> &String {
        &self.sad_value_type
    }

    fn deser(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        let mut coll = Vec::<Vec<SadValue>>::new();
        let count = u32::try_from_slice(&data[0..4]).unwrap();
        *data = &data[4..];
        for _ in 0..count {
            let mut spare = Vec::<SadValue>::new();
            for c in &self.children {
                c.deser(data, &mut spare);
            }
            coll.push(spare);
        }
        collection.push(SadValue::HashMap(coll))
    }
}

impl NodeWithChildren for SadHashMap {
    fn children(&self) -> &Vec<Box<dyn Node>> {
        &self.children
    }
}

/// Implements NodeWithChildren for Structure (i.e. Rust struct)
#[derive(Debug)]
pub struct SadStructure {
    sad_value_type: String,
    children: Vec<Box<dyn Node>>,
}

impl SadStructure {
    fn from_yaml(in_yaml: &Yaml) -> SadTreeResult<Box<dyn Node>> {
        let in_str = in_yaml[SAD_YAML_TYPE].as_str().unwrap();
        let mut array = Vec::<Box<dyn Node>>::new();
        let fields = &in_yaml[SAD_YAML_FIELDS];

        match fields {
            Yaml::Array(lst) => {
                for hl in lst {
                    array.push(parse(hl).unwrap())
                }
                Ok(Box::new(SadStructure {
                    sad_value_type: String::from(in_str),
                    children: array,
                }))
            }
            _ => Err(SadTreeError::ExpectedCStructFields),
        }
    }
}
impl Node for SadStructure {
    fn decl_type(&self) -> &String {
        &self.sad_value_type
    }

    fn deser(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        let mut coll = Vec::<SadValue>::new();
        for c in &self.children {
            c.deser(data, &mut coll)
        }
        collection.push(SadValue::CStruct(coll))
    }
}

impl NodeWithChildren for SadStructure {
    fn children(&self) -> &Vec<Box<dyn Node>> {
        &self.children
    }
}

/// Implements NodeWithChildren for Vector (i.e. Rust Vec)
#[derive(Debug)]
pub struct SadVector {
    sad_value_type: String,
    children: Vec<Box<dyn Node>>,
}

impl SadVector {
    fn from_yaml(in_yaml: &Yaml) -> SadTreeResult<Box<dyn Node>> {
        let in_str = in_yaml[SAD_YAML_TYPE].as_str().unwrap();
        let mut array = Vec::<Box<dyn Node>>::new();
        let contains = &in_yaml[SAD_YAML_CONTAINS];
        match contains {
            Yaml::Array(lst) => {
                for hl in lst {
                    array.push(parse(hl).unwrap())
                }
                Ok(Box::new(SadVector {
                    sad_value_type: String::from(in_str),
                    children: array,
                }))
            }
            _ => Err(SadTreeError::ExpectedVecContains),
        }
    }
}
impl Node for SadVector {
    fn decl_type(&self) -> &String {
        &self.sad_value_type
    }

    fn deser(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        // let mut coll = Vec::<Vec<SadValue>>::new();
        let count = u32::try_from_slice(&data[0..4]).unwrap();
        *data = &data[4..];
        let mut spare = Vec::<SadValue>::new();
        for _ in 0..count {
            for c in &self.children {
                c.deser(data, &mut spare);
            }
        }
        collection.push(SadValue::Vec(spare));
    }
}

impl NodeWithChildren for SadVector {
    fn children(&self) -> &Vec<Box<dyn Node>> {
        &self.children
    }
}

/// Implements NodeWithChildren for Tuple (i.e. Rust tuple)
#[derive(Debug)]
pub struct SadTuple {
    sad_value_type: String,
    children: Vec<Box<dyn Node>>,
}

impl SadTuple {
    fn from_yaml(in_yaml: &Yaml) -> SadTreeResult<Box<dyn Node>> {
        let in_str = in_yaml[SAD_YAML_TYPE].as_str().unwrap();
        let mut array = Vec::<Box<dyn Node>>::new();
        let fields = &in_yaml[SAD_YAML_FIELDS];
        match fields {
            Yaml::Array(lst) => {
                for hl in lst {
                    array.push(parse(hl).unwrap())
                }
                Ok(Box::new(SadTuple {
                    sad_value_type: String::from(in_str),
                    children: array,
                }))
            }
            _ => Err(SadTreeError::ExpectedTupleFields),
        }
    }
}
impl Node for SadTuple {
    fn decl_type(&self) -> &String {
        &self.sad_value_type
    }

    fn deser(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        for c in &self.children {
            c.deser(data, collection)
        }
    }
}

impl NodeWithChildren for SadTuple {
    fn children(&self) -> &Vec<Box<dyn Node>> {
        &self.children
    }
}

/// Implements NodeWithChildren for SadTree which holds
/// the YAML parse tree for deserialization
#[derive(Debug)]
pub struct SadTree {
    yaml_decl_type: String,
    name: String,
    varnames: Vec<String>,
    children: Vec<Box<dyn Node>>,
}

impl SadTree {
    pub fn new(in_yaml: &Yaml) -> SadTreeResult<Self> {
        let mut array = Vec::<Box<dyn Node>>::new();
        let mut vars = Vec::<String>::new();
        match &*in_yaml {
            Yaml::Hash(ref hmap) => {
                let (key, value) = hmap.front().unwrap();
                match value {
                    Yaml::Array(hlobjects) => {
                        for hl in hlobjects {
                            let (varname, h1_value) = hl.as_hash().unwrap().front().unwrap();

                            vars.push(varname.as_str().unwrap().to_string());
                            array.push(parse(h1_value).unwrap());
                        }
                        Ok(Self {
                            yaml_decl_type: String::from("tree"),
                            name: key.as_str().unwrap().to_string(),
                            varnames: vars,
                            children: array,
                        })
                    }
                    _ => Err(SadTreeError::ExpectedArray),
                }
            }
            _ => Err(SadTreeError::ExpectedHashMap),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

impl NodeWithChildren for SadTree {
    fn children(&self) -> &Vec<Box<dyn Node>> {
        &self.children
    }
}

impl Node for SadTree {
    fn decl_type(&self) -> &String {
        &self.yaml_decl_type
    }

    fn deser(&self, data: &mut &[u8], collection: &mut Vec<SadValue>) {
        for c in &self.children {
            c.deser(data, collection)
        }
    }
}

/// Public struct for interacting deserialization to YAML construct declarations
#[derive(Debug)]
pub struct Deseriaizer<'a> {
    yaml_declaration: &'a Yaml,
    sad_tree: SadTree,
}

impl<'a> Deseriaizer<'a> {
    pub fn new(in_yaml: &'a Yaml) -> Self {
        Self {
            yaml_declaration: in_yaml,
            sad_tree: SadTree::new(in_yaml).unwrap(),
        }
    }
    pub fn deser(&self, data: &mut &[u8]) -> SadTreeResult<Vec<SadValue>> {
        // let inbound = data.to_vec();
        let mut hm = Vec::<SadValue>::new();
        self.tree().deser(data, &mut hm);
        Ok(hm)
    }
    pub fn tree(&self) -> &SadTree {
        &self.sad_tree
    }
}

/// Dispatches YAML parse Node types
fn parse(in_yaml: &Yaml) -> Result<Box<dyn Node>, SadTreeError> {
    if let Some(in_type_key) = &mut in_yaml
        .as_hash()
        .unwrap()
        .get(SAD_TYPE_JSON.first().unwrap())
    {
        if let Some(s) = JUMP_TABLE.get(in_type_key.as_str().unwrap()) {
            s(in_yaml)
        } else {
            SAD_JUMP_OTHER(in_yaml)
        }
    } else {
        Err(SadTreeError::ExpectedTypeKeyError(
            SAD_YAML_TYPE.to_string(),
        ))
    }
}

fn deser_nodes(node: &Box<dyn Node>, collect: &mut Vec<String>) {
    match node.decl_type().as_str() {
        "length_prefix" => {
            let lp = node.downcast_ref::<SadLengthPrefix>().unwrap();
            collect.push(lp.length_type().to_string());
            for c in lp.children() {
                deser_nodes(c, collect)
            }
        }
        "HashMap" => {
            let lp = node.downcast_ref::<SadHashMap>().unwrap();
            collect.push(lp.decl_type().to_string());
            for c in lp.children() {
                deser_nodes(c, collect)
            }
        }
        "Vec" => {
            let lp = node.downcast_ref::<SadVector>().unwrap();
            collect.push(lp.decl_type().to_string());
            for c in lp.children() {
                deser_nodes(c, collect)
            }
        }
        "Tuple" => {
            let lp = node.downcast_ref::<SadTuple>().unwrap();
            collect.push(lp.decl_type().to_string());
            for c in lp.children() {
                deser_nodes(c, collect)
            }
        }
        "CStruct" => {
            let lp = node.downcast_ref::<SadStructure>().unwrap();
            collect.push(lp.decl_type().to_string());
            for c in lp.children() {
                deser_nodes(c, collect)
            }
        }
        "NamedField" => {
            let lp = node.downcast_ref::<SadNamedField>().unwrap();
            collect.push(lp.name().to_string());
            for c in lp.children() {
                deser_nodes(c, collect)
            }
        }
        _ => collect.push(node.decl_type().to_string()),
    }
}

pub fn deser_tree_decls(tree: &SadTree) -> Vec<(String, Vec<String>)> {
    let mut collection = Vec::<(String, Vec<String>)>::new();
    let mut index = 0;
    for c in tree.children() {
        let varname = tree.varnames.get(index).unwrap();
        let mut cset = Vec::<String>::new();
        deser_nodes(c, &mut cset);
        collection.push((varname.to_string(), cset));
        index += 1;
    }
    collection
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use base64::decode;
    use borsh::BorshSerialize;
    use gadgets_common::load_yaml_file;
    use solana_sdk::pubkey::Pubkey;
    use strum::VariantNames;
    use yaml_rust::YamlLoader;

    const INDEX_HASHMAP_STRING_U128: usize = 2;
    const INDEX_LENGTHPREFIX_HASHMAP: usize = 3;
    const INDEX_VECTOR_STRING: usize = 4;
    const INDEX_VECTOR_U32: usize = 5;
    const INDEX_TUPLE_STRING_U128: usize = 6;
    const INDEX_STRUCT_STRING_U32: usize = 7;
    const INDEX_STRUCT_STRING_VECTOR_U32: usize = 8;
    const INDEX_VECTOR_VECTOR_STRING: usize = 9;
    const INDEX_PUBLICKEY: usize = 10;

    #[derive(BorshSerialize)]
    struct OfTuple(String, u128);

    #[derive(BorshSerialize)]
    struct OfStruct {
        name: String,
        age: u32,
    }
    /// vscode changes cwd depending on running test or debugging test
    fn get_runner_yaml() -> Vec<Yaml> {
        if std::env::current_dir().unwrap().ends_with("sad") {
            load_yaml_file("../yaml_samps/runner.yml").unwrap()
        } else {
            load_yaml_file("./yaml_samps/runner.yml").unwrap()
        }
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

    fn walkage_print(node: &Box<dyn Node>, indent: &mut usize) {
        match node.decl_type().as_str() {
            "length_prefix" => {
                let lp = node.downcast_ref::<SadLengthPrefix>().unwrap();
                // println!("{:indent$}{}", "", lp.decl_type(), indent = *indent);
                let y = lp.length_type();
                *indent += 2;
                println!("{:indent$}{}", "", y, indent = *indent);
                for c in lp.children() {
                    walkage_print(c, indent)
                }
                *indent -= 2
            }
            "HashMap" => {
                let lp = node.downcast_ref::<SadHashMap>().unwrap();
                println!("{:indent$}{}", "", lp.decl_type(), indent = *indent);
                *indent += 2;
                for c in lp.children() {
                    walkage_print(c, indent)
                }
                *indent -= 2
            }
            "Vec" => {
                let lp = node.downcast_ref::<SadVector>().unwrap();
                println!("{:indent$}{}", "", lp.decl_type(), indent = *indent);
                *indent += 2;
                for c in lp.children() {
                    walkage_print(c, indent)
                }
                *indent -= 2
            }
            "Tuple" => {
                let lp = node.downcast_ref::<SadTuple>().unwrap();
                println!("{:indent$}{}", "", lp.decl_type(), indent = *indent);
                *indent += 2;
                for c in lp.children() {
                    walkage_print(c, indent)
                }
                *indent -= 2
            }
            "CStruct" => {
                let lp = node.downcast_ref::<SadStructure>().unwrap();
                println!("{:indent$}{}", "", lp.decl_type(), indent = *indent);
                *indent += 2;
                for c in lp.children() {
                    walkage_print(c, indent)
                }
                *indent -= 2
            }
            "NamedField" => {
                let lp = node.downcast_ref::<SadNamedField>().unwrap();
                println!("{:indent$}{}", "", lp.name(), indent = *indent);
                *indent += 2;
                for c in lp.children() {
                    walkage_print(c, indent)
                }
                *indent -= 2
            }
            _ => println!("{:indent$}{}", "", node.decl_type(), indent = *indent),
        }
    }

    fn walk_tree_print(tree: &SadTree) {
        let mut indent: usize = 0;
        println!(
            "{:indent$}{}: {}",
            "",
            tree.decl_type(),
            tree.get_name(),
            indent = indent
        );

        let mut index = 0 as usize;
        for c in tree.children() {
            let varname = tree.varnames.get(index).unwrap();
            print!("{}: -> ", varname);
            index += 1;
            walkage_print(c, &mut indent);
            indent = 0
        }
    }
    #[test]
    fn test_deserialization_pass() {
        println!("{:?}", std::env::current_dir().unwrap());
        let pacc = "ASUAAAABAAAABAAAAEFLZXkVAAAATWludGVkIGtleSB2YWx1ZSBwYWlyAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==";
        let pacv = decode(pacc).unwrap();
        let result = get_sample_yaml();
        let desc = Deseriaizer::new(&result[0]);
        println!("{:?}", deser_tree_decls(desc.tree()));
        // walk_tree_print(desc.tree());
        let deserialize_vector = desc.deser(&mut pacv.as_slice());
        println!("{:?}", deserialize_vector.unwrap());
    }

    #[test]
    fn test_leaf_node_pass() {
        for v in SadValue::VARIANTS.iter() {
            let vs = *v;
            let d = format!("{}: {}", "type", vs);
            let docs = YamlLoader::load_from_str(&d).unwrap();
            let doc = &docs[0]; // select the first document
            let sl = SadLeaf::from_yaml(doc);
            assert!(sl.is_ok());
            println!("{:?}", sl);
        }
    }

    #[test]
    fn test_to_string() {
        let docs = YamlLoader::load_from_str(&format!("{}", SAD_YAML_TYPE)).unwrap();
        println!("{:?}", docs);
    }

    #[test]
    fn test_scalars_pass() {
        let mut pos = 0;
        let pos_end = 14;
        for v in SadValue::VARIANTS.iter() {
            if pos == pos_end {
                break;
            }
            pos += 1;
            let vs = *v;
            let d = format!("{}: {}", "type", vs);
            let docs = YamlLoader::load_from_str(&d).unwrap();
            let result = parse(&docs[0]);
            assert!(result.is_ok());
        }
    }
    #[test]
    fn test_runner_pass() {
        let result = get_runner_yaml();
        for body in result {
            let desc = Deseriaizer::new(&body);
            println!("{:?}", deser_tree_decls(desc.tree()));
            // walk_tree_print(desc.tree());
        }
    }

    #[test]
    fn test_hashmap_pass() {
        let mut mhmap = HashMap::<&str, u128>::new();
        mhmap.insert("foo", 1u128);
        mhmap.insert("bar", 2u128);
        mhmap.insert("baz", 3u128);
        let result = get_runner_yaml();
        let desc = Deseriaizer::new(&result[INDEX_HASHMAP_STRING_U128]);
        let data = mhmap.try_to_vec().unwrap();
        let deserialize_vector = desc.deser(&mut data.as_slice());
        println!("{:?}", deserialize_vector.unwrap());
    }
    #[test]
    fn test_length_prefix_hashmap_pass() {
        let mut mhmap = HashMap::<&str, &str>::new();
        mhmap.insert("foo", "1u128");
        mhmap.insert("bar", "2u128");
        mhmap.insert("baz", "3u128");
        let result = get_runner_yaml();
        let desc = Deseriaizer::new(&result[INDEX_LENGTHPREFIX_HASHMAP]);
        let data = mhmap.try_to_vec().unwrap();
        let lpref = data.len() as u32;
        let mut head = lpref.try_to_vec().unwrap();
        head.extend(data);
        let deserialize_vector = desc.deser(&mut head.as_slice());
        println!("{:?}", deserialize_vector.unwrap());
    }

    #[test]
    fn test_vector_string_pass() {
        let mut mhmap = Vec::<String>::new();
        mhmap.push(String::from("foo"));
        mhmap.push(String::from("bar"));
        let result = get_runner_yaml();
        let desc = Deseriaizer::new(&result[INDEX_VECTOR_STRING]);
        let data = mhmap.try_to_vec().unwrap();
        let deserialize_vector = desc.deser(&mut data.as_slice());
        println!("{:?}", deserialize_vector.unwrap());
    }

    #[test]
    fn test_vector_u32_pass() {
        let mut mhmap = Vec::<u32>::new();
        mhmap.push(1u32);
        mhmap.push(2u32);
        let result = get_runner_yaml();
        let desc = Deseriaizer::new(&result[INDEX_VECTOR_U32]);
        let data = mhmap.try_to_vec().unwrap();
        let deserialize_vector = desc.deser(&mut data.as_slice());
        println!("{:?}", deserialize_vector.unwrap());
    }

    #[test]
    fn test_tuple_pass() {
        let mhmap = OfTuple("Foo".to_string(), 19u128);
        let result = get_runner_yaml();
        let desc = Deseriaizer::new(&result[INDEX_TUPLE_STRING_U128]);
        walk_tree_print(desc.tree());
        let data = mhmap.try_to_vec().unwrap();
        println!("{:?}", data);
        let deserialize_vector = desc.deser(&mut data.as_slice());
        println!("{:?}", deserialize_vector.unwrap());
    }

    #[test]
    fn test_struct_pass() {
        let mhmap = OfStruct {
            name: "Frank".to_string(),
            age: 64,
        };
        let result = get_runner_yaml();
        let desc = Deseriaizer::new(&result[INDEX_STRUCT_STRING_U32]);
        let data = mhmap.try_to_vec().unwrap();
        println!("{:?}", data);
        let deserialize_vector = desc.deser(&mut data.as_slice());
        println!("{:?}", deserialize_vector.unwrap());
    }
    #[test]
    fn pubkey_pass() {
        let result = get_runner_yaml();
        let desc = Deseriaizer::new(&result[INDEX_PUBLICKEY]);
        walk_tree_print(desc.tree());
        let pk = Pubkey::from_str("A94wMjV54C8f8wn7zL8TxNCdNiGoq7XSN7vWGrtd4vwU").unwrap();
        let pk_ser = pk.try_to_vec().unwrap();
        println!("{:?}", pk_ser);
        let deserialize_vector = desc.deser(&mut pk_ser.as_slice());
        println!("{:?}", deserialize_vector.unwrap());
    }
}
