//! @brief Heart of deserialization
//! Constructs for instantiating parse tree with constructs
//! from YAML declaration and then walking said tree to deserialize
//! input vector of bytes that come from a program owned account data

use {
    crate::{
        errors::{SadTreeError, SadTreeResult},
        sadtypes::{deser_value_for, is_sadvalue_type, SadValue},
    },
    borsh::BorshDeserialize,
    lazy_static::*,
    std::collections::HashMap,
    yaml_rust::yaml::Yaml,
};
/// Simple Node for tree membership
trait Node: std::fmt::Debug {
    /// Clone of the inbound yaml sad 'type'
    fn decl_type(&self) -> &String;
    fn deser(&self, data: &mut &[u8], collection: &mut Vec<SadValue>);
}
/// Simple branch for tree membership
trait NodeWithChildren: Node {
    fn children(&self) -> &Vec<Box<dyn Node>>;
}

const SAD_YAML_TYPE: &str = "type";
const SAD_YAML_NAME: &str = "name";
const SAD_YAML_DESCRIPTOR: &str = "descriptor";
const SAD_YAML_SIZE_TYPE: &str = "size_type";
const SAD_YAML_CONTAINS: &str = "contains";
const SAD_YAML_FIELDS: &str = "fields";

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
        array.push(parse(desc).unwrap());
        Ok(Box::new(SadNamedField {
            sad_field_name: String::from(in_name),
            sad_value_type: String::from(SAD_YAML_DESCRIPTOR),
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
        if !is_sadvalue_type(in_str) {
            return Err(SadTreeError::UnknownType(String::from(in_str)));
        }
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
        if !is_sadvalue_type(in_str) {
            return Err(SadTreeError::UnknownType(String::from(in_str)));
        }

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
        if !is_sadvalue_type(in_str) {
            return Err(SadTreeError::UnknownType(String::from(in_str)));
        }

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
        if !is_sadvalue_type(in_str) {
            return Err(SadTreeError::UnknownType(String::from(in_str)));
        }

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
    children: Vec<Box<dyn Node>>,
}

impl SadTree {
    pub fn new(in_yaml: &Yaml) -> SadTreeResult<Self> {
        let mut array = Vec::<Box<dyn Node>>::new();
        match &*in_yaml {
            Yaml::Hash(ref hmap) => {
                let (key, value) = hmap.front().unwrap();
                match value {
                    Yaml::Array(hlobjects) => {
                        for hl in hlobjects {
                            let (_, h1_value) = hl.as_hash().unwrap().front().unwrap();
                            array.push(parse(h1_value).unwrap());
                        }
                        Ok(Self {
                            yaml_decl_type: String::from("tree"),
                            children: array,
                            name: key.as_str().unwrap().to_string(),
                        })
                    }
                    _ => Err(SadTreeError::ExpectedArray),
                }
            }
            _ => Err(SadTreeError::ExpectedHashMap),
        }
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
        let inbound = data.to_vec();
        let mut hm = Vec::<SadValue>::new();
        self.tree().deser(data, &mut hm);
        Ok(hm)
    }
    fn tree(&self) -> &SadTree {
        &self.sad_tree
    }
}

// Jump table for generalizing parse construction
lazy_static! {
    static ref JUMP_TABLE: HashMap<String, fn(&Yaml) -> Result<Box<dyn Node>, SadTreeError>> = {
        let mut jump_table =
            HashMap::<String, fn(&Yaml) -> Result<Box<dyn Node>, SadTreeError>>::new();
        jump_table.insert("length_prefix".to_string(), SadLengthPrefix::from_yaml);
        jump_table.insert("HashMap".to_string(), SadHashMap::from_yaml);
        jump_table.insert("Vec".to_string(), SadVector::from_yaml);
        jump_table.insert("Tuple".to_string(), SadTuple::from_yaml);
        jump_table.insert("CStruct".to_string(), SadStructure::from_yaml);
        jump_table.insert("NamedField".to_string(), SadNamedField::from_yaml);
        jump_table.insert("other".to_string(), SadLeaf::from_yaml);
        jump_table
    };
}

/// Dispatches YAML parse Node types
fn parse(in_yaml: &Yaml) -> Result<Box<dyn Node>, SadTreeError> {
    let default = JUMP_TABLE.get("other").unwrap();
    // Expects a Hash construct and first entry
    let type_in = in_yaml.as_hash().unwrap().front().unwrap().1;
    if let Some(s) = JUMP_TABLE.get(type_in.as_str().unwrap()) {
        s(in_yaml)
    } else {
        default(in_yaml)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::decode;
    use borsh::BorshSerialize;
    use gadgets_common::load_yaml_file;
    use strum::VariantNames;
    use yaml_rust::YamlLoader;

    const SCLI: &str = "../../samples/yamldecls/SampGgdt3wioaoMZhC6LTSbg4pnuvQnSfJpDYeuXQBv.yml";

    const INDEX_HASHMAP_STRING_U128: usize = 2;
    const INDEX_LENGTHPREFIX_HASHMAP: usize = 3;
    const INDEX_VECTOR_STRING: usize = 4;
    const INDEX_VECTOR_U32: usize = 5;
    const INDEX_TUPLE_STRING_U128: usize = 6;
    const INDEX_STRUCT_STRING_U32: usize = 7;

    #[derive(BorshSerialize)]
    struct OfTuple(String, u128);

    #[derive(BorshSerialize)]
    struct OfStruct {
        name: String,
        age: u32,
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
        let result = load_yaml_file("../yaml_samps/runner.yml").unwrap();
        for body in result {
            println!("{:?}", Deseriaizer::new(&body).tree());
        }
    }

    #[test]
    fn test_hashmap_pass() {
        let mut mhmap = HashMap::<&str, u128>::new();
        mhmap.insert("foo", 1u128);
        mhmap.insert("bar", 2u128);
        mhmap.insert("baz", 3u128);
        let result = load_yaml_file("../yaml_samps/runner.yml").unwrap();
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
        let result = load_yaml_file("../yaml_samps/runner.yml").unwrap();
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
        let result = load_yaml_file("../yaml_samps/runner.yml").unwrap();
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
        let result = load_yaml_file("../yaml_samps/runner.yml").unwrap();
        let desc = Deseriaizer::new(&result[INDEX_VECTOR_U32]);
        let data = mhmap.try_to_vec().unwrap();
        let deserialize_vector = desc.deser(&mut data.as_slice());
        println!("{:?}", deserialize_vector.unwrap());
    }

    #[test]
    fn test_tuple_pass() {
        let mhmap = OfTuple("Foo".to_string(), 19u128);
        let result = load_yaml_file("../yaml_samps/runner.yml").unwrap();
        let desc = Deseriaizer::new(&result[INDEX_TUPLE_STRING_U128]);
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
        let result = load_yaml_file("../yaml_samps/runner.yml").unwrap();
        let desc = Deseriaizer::new(&result[INDEX_STRUCT_STRING_U32]);
        let data = mhmap.try_to_vec().unwrap();
        println!("{:?}", data);
        let deserialize_vector = desc.deser(&mut data.as_slice());
        println!("{:?}", deserialize_vector.unwrap());
    }

    #[test]
    fn test_deserialization_pass() {
        let pacc = "ASUAAAABAAAABAAAAEFLZXkVAAAATWludGVkIGtleSB2YWx1ZSBwYWlyAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==";
        let pacv = decode(pacc).unwrap();
        let result = load_yaml_file(SCLI).unwrap();
        let desc = Deseriaizer::new(&result[0]);
        let deserialize_vector = desc.deser(&mut pacv.as_slice());
        println!("{:?}", deserialize_vector.unwrap());
    }
}
