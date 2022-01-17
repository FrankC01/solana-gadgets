//! @brief Heart of deserialization
//! Constructs for instantiating parse tree with constructs
//! from YAML declaration and then walking said tree to deserialize
//! input vector of bytes that come from a program owned account data

use {
    crate::{
        errors::{SadTreeError, SadTreeResult},
        sadtypes::{deser_value_for, from_scalar_value_for, is_sadvalue_type, SadValue},
    },
    borsh::BorshDeserialize,
    downcast_rs::{impl_downcast, Downcast},
    lazy_static::*,
    serde_json::{json, Value},
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
// const SAD_YAML_SIZE_TYPE: &str = "size_type";
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
}

impl SadPublicKey {
    fn from_yaml(in_yaml: &Yaml) -> SadTreeResult<Box<dyn Node>> {
        let in_str = in_yaml[SAD_YAML_TYPE].as_str().unwrap();
        if is_sadvalue_type(in_str) {
            Ok(Box::new(SadPublicKey {
                sad_value_type: String::from(in_str),
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
        let mut spare = Vec::<SadValue>::new();
        for c in &self.children {
            c.deser(data, &mut spare)
        }
        collection.push(SadValue::Tuple(spare));
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

#[derive(Debug, PartialEq, Default)]
/// SadSchemaElement describes the concrete element(s) (e.g. Scalar vs. Compound)
/// and will contain nested elements if compound
struct SadSchemaElement {
    schema_type: String,
    schema_ancillary_type: Option<String>,
    scalar: bool,
    items: Option<Vec<SadSchemaElement>>,
}

impl SadSchemaElement {
    fn new(schema_type_name: &String) -> Self {
        Self {
            schema_type: schema_type_name.clone(),
            ..SadSchemaElement::default()
        }
    }
    fn ancillary_type(mut self, atype: &String) -> Self {
        self.schema_ancillary_type = Some(atype.clone());
        self
    }

    fn scalar(mut self, scal: bool) -> Self {
        self.scalar = scal;
        self
    }
    fn items(mut self, elements: Vec<SadSchemaElement>) -> Self {
        self.items = Some(elements);
        self
    }
    pub fn is_scalar(&self) -> bool {
        self.scalar
    }

    pub fn sad_to_json(&self, for_data: &Vec<SadValue>) -> Value {
        if let Some(items) = &self.items {
            match self.schema_type.as_str() {
                "Vec" => {
                    match &for_data[0] {
                        // Get the inner vector and, using the data size, repeat
                        SadValue::Vec(v) => {
                            let mut json_vec = json!([]);
                            let raw_json = json_vec.as_array_mut().unwrap();
                            for i in 0..v.len() {
                                let mut d = Vec::<SadValue>::new();
                                d.push(v[i].clone());
                                raw_json.push(items[0].sad_to_json(&d))
                            }
                            json_vec
                        }
                        _ => unreachable!(),
                    }
                }
                "Tuple" => match &for_data[0] {
                    SadValue::Tuple(v) => {
                        let mut json_tuple = json!([]);
                        let raw_json = json_tuple.as_array_mut().unwrap();
                        for i in 0..v.len() {
                            let mut d = Vec::<SadValue>::new();
                            d.push(v[i].clone());
                            raw_json.push(items[i].sad_to_json(&d))
                        }
                        json_tuple
                    }
                    _ => unreachable!(),
                },
                "HashMap" => match &for_data[0] {
                    SadValue::HashMap(v) => {
                        let mut json_map = json!({});
                        let raw_json = json_map.as_object_mut().unwrap();
                        for vi in 0..v.len() {
                            let mut d = Vec::<SadValue>::new();
                            let json_key = if items[0].is_scalar()
                                && items[0].schema_type == "String".to_string()
                            {
                                from_scalar_value_for(&v[vi][0])
                            } else {
                                d.push(v[vi][0].clone());
                                let sad_key = items[0].sad_to_json(&d);
                                d.drain(..);
                                sad_key.to_string()
                            };
                            d.push(v[vi][1].clone());
                            let json_value = items[1].sad_to_json(&d);
                            raw_json.insert(json_key, json_value);
                        }
                        json_map
                    }
                    _ => unreachable!(),
                },
                "CStruct" => match &for_data[0] {
                    SadValue::CStruct(nfs) => {
                        let mut json_cstruct = json!({});
                        let raw_json = json_cstruct.as_object_mut().unwrap();
                        println!(
                            "CStruct items len = {} data len = {}",
                            items.len(),
                            nfs.len()
                        );
                        for i in 0..nfs.len() {
                            match &nfs[i] {
                                SadValue::NamedField(nvp) => {
                                    let mut d = Vec::<SadValue>::new();
                                    d.push(nfs[i].clone());
                                    raw_json.insert(
                                        from_scalar_value_for(&nvp[0]),
                                        items[i].sad_to_json(&d),
                                    );
                                }
                                _ => unreachable!(),
                            }
                        }
                        json_cstruct
                    }
                    _ => unreachable!(),
                },
                "NamedField" => match &for_data[0] {
                    SadValue::NamedField(f) => {
                        let mut d = Vec::<SadValue>::new();
                        d.push(f[1].clone());
                        items[0].sad_to_json(&d)
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        } else {
            match &for_data[0] {
                SadValue::String(item) => json!(item),
                SadValue::Bool(item) => json!(item),
                SadValue::U8(item) => json!(item),
                SadValue::U16(item) => json!(item),
                SadValue::U32(item) => json!(item),
                SadValue::U64(item) => json!(item),
                SadValue::U128(item) => json!(item.to_string()),
                SadValue::I8(item) => json!(item),
                SadValue::I16(item) => json!(item),
                SadValue::I32(item) => json!(item),
                SadValue::I64(item) => json!(item),
                SadValue::I128(item) => json!(item.to_string()),
                SadValue::F32(item) => json!(item),
                SadValue::F64(item) => json!(item),
                SadValue::PublicKey(item) => json!(item.to_string()),
                _ => unreachable!(),
            }
        }
    }
}

#[derive(Debug, PartialEq, Default)]
/// SchemaItem represents a top level entity in the
/// deserialization YAML. It contains the SadSchemaElements
struct SadSchemaItem {
    item_name: String,
    items: Vec<SadSchemaElement>,
}

impl SadSchemaItem {
    pub fn get_name(&self) -> &String {
        &self.item_name
    }

    pub fn get_items(&self) -> &Vec<SadSchemaElement> {
        &self.items
    }

    fn sad_to_json(&self, json_map: &mut Value, for_data: &Vec<SadValue>) {
        let raw_map = json_map.as_object_mut().unwrap();
        raw_map.insert(
            self.get_name().clone(),
            self.get_items()[0].sad_to_json(for_data),
        );
    }
}

#[derive(Debug, PartialEq)]
pub struct SadSchema {
    item_names: Vec<String>,
    item_type_prefix: Vec<String>,
    items: Vec<SadSchemaItem>,
}

impl SadSchema {
    fn get_items(&self) -> &Vec<SadSchemaItem> {
        &self.items
    }

    fn get_item_names(&self) -> &Vec<String> {
        &self.item_names
    }

    /// Reference to item_name_TYPE labels
    pub fn item_type_prefixes(&self) -> &Vec<String> {
        &self.item_type_prefix
    }

    /// Gets item_name_type constructs, a Vec of
    /// item_name_TYPE
    fn gen_items_prefix(items: &Vec<SadSchemaItem>) -> Vec<String> {
        let mut itypes = Vec::<String>::new();
        for item in items {
            if item.get_items().len() == 1 {
                itypes.push(format!(
                    "{}_{}",
                    item.get_name(),
                    item.get_items()[0].schema_type
                ))
            } else {
                panic!()
            }
        }
        itypes
    }

    /// Given a header string vector, generate a summary
    /// with item_type keys and hit count: HashMap<String, u64>
    pub fn gen_type_summary(&self, inhdr: &Vec<String>) -> HashMap<String, u64> {
        let mut summary = HashMap::<String, u64>::new();
        let mut keys = Vec::<&String>::new();
        for k in self.item_type_prefixes() {
            summary.insert(k.to_string(), 0u64);
            keys.push(k);
        }
        for t in inhdr {
            for k in keys.iter() {
                if t.contains(*k) {
                    *summary.entry(k.to_string()).or_insert(0) += 1;
                }
            }
        }
        summary
    }

    /// Given a result of deserialization, generate a JSON
    /// representation

    fn sad_to_json(&self, with_data: &Vec<SadValue>) -> Value {
        let mut json_out = json!({});
        let mut index = 0usize;
        for item in self.get_items() {
            let mut d = Vec::<SadValue>::new();
            d.push(with_data[index].clone());
            item.sad_to_json(&mut json_out, &d);
            index += 1;
        }
        json_out
    }

    fn itemize(children: &Vec<Box<dyn Node>>) -> Vec<SadSchemaElement> {
        let mut items = Vec::<SadSchemaElement>::new();
        for c in children {
            SadSchema::schema_item(c, &mut items)
        }
        items
    }

    fn schema_item(node: &Box<dyn Node>, collect: &mut Vec<SadSchemaElement>) {
        let schm_element = SadSchemaElement::new(&node.decl_type());
        match node.decl_type().as_str() {
            "HashMap" => {
                let lp = node.downcast_ref::<SadHashMap>().unwrap();
                collect.push(
                    schm_element
                        .scalar(false)
                        .items(SadSchema::itemize(lp.children())),
                )
            }
            "Vec" => {
                let lp = node.downcast_ref::<SadVector>().unwrap();
                collect.push(
                    schm_element
                        .scalar(false)
                        .items(SadSchema::itemize(lp.children())),
                )
            }
            "Tuple" => {
                let lp = node.downcast_ref::<SadTuple>().unwrap();
                collect.push(
                    schm_element
                        .scalar(false)
                        .items(SadSchema::itemize(lp.children())),
                )
            }
            "CStruct" => {
                let lp = node.downcast_ref::<SadStructure>().unwrap();

                collect.push(
                    schm_element
                        .scalar(false)
                        .items(SadSchema::itemize(lp.children())),
                )
            }
            "NamedField" => {
                let lp = node.downcast_ref::<SadNamedField>().unwrap();
                collect.push(
                    schm_element
                        .ancillary_type(&lp.sad_field_name)
                        .scalar(false)
                        .items(SadSchema::itemize(lp.children())),
                )
            }
            _ => collect.push(schm_element.scalar(true)),
        }
    }

    fn schema_for(name_id: String, node: &Box<dyn Node>) -> SadSchemaItem {
        let mut ssi = Vec::<SadSchemaElement>::new();
        SadSchema::schema_item(node, &mut ssi);
        SadSchemaItem {
            item_name: name_id,
            items: ssi,
        }
    }

    fn schema(tree: &SadTree) -> SadSchema {
        let mut vi = Vec::<SadSchemaItem>::new();
        let mut vn = Vec::<String>::new();
        let mut index = 0;
        for node in tree.children() {
            let item_name = tree.varnames.get(index).unwrap().to_string();
            let item = SadSchema::schema_for(item_name.clone(), node);
            vi.push(item);
            vn.push(item_name);
            index += 1;
        }

        SadSchema {
            item_type_prefix: SadSchema::gen_items_prefix(&vi),
            items: vi,
            item_names: vn,
        }
    }
}

/// Public struct for interacting deserialization to YAML construct declarations
#[derive(Debug)]
pub struct Deseriaizer {
    sad_schema: SadSchema,
    sad_tree: SadTree,
}

impl Deseriaizer {
    pub fn new(in_yaml: &Yaml) -> Self {
        let tree = SadTree::new(in_yaml).unwrap();
        let scm = SadSchema::schema(&tree);
        Self {
            sad_tree: tree,
            sad_schema: scm,
        }
    }

    pub fn schema(&self) -> &SadSchema {
        &self.sad_schema
    }

    pub fn to_json(&self, with_data: &Vec<SadValue>) -> Value {
        self.sad_schema.sad_to_json(&with_data)
    }

    pub fn deser(&self, data: &mut &[u8]) -> SadTreeResult<Vec<SadValue>> {
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
    const INDEX_VECTOR_STRING: usize = 3;
    const INDEX_VECTOR_U32: usize = 4;
    const INDEX_TUPLE_STRING_U128: usize = 5;
    const INDEX_STRUCT_STRING_U32: usize = 6;
    const INDEX_PUBLICKEY: usize = 9;

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

    #[test]
    fn test_deserialization_pass() {
        println!("{:?}", std::env::current_dir().unwrap());
        let pacc = "ASUAAAABAAAABAAAAEFLZXkVAAAATWludGVkIGtleSB2YWx1ZSBwYWlyAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==";
        let pacv = decode(pacc).unwrap();
        let result = get_sample_yaml();
        let desc = Deseriaizer::new(&result[0]);
        // println!("{:?}", desc.schema().flat_header(None));
        let deserialize_vector = desc.deser(&mut pacv.as_slice()).unwrap();
        println!("{:?}", deserialize_vector);
        println!(
            "{:?}",
            desc.schema().sad_to_json(&deserialize_vector).to_string()
        );
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
            // println!("{:?}", desc.schema().flat_header(None));
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
        let deserialize_vector = desc.deser(&mut data.as_slice()).unwrap();
        println!("{:?}", deserialize_vector);
        println!(
            "{}",
            serde_json::to_string_pretty(&desc.schema().sad_to_json(&deserialize_vector)).unwrap()
        );
    }

    #[test]
    fn test_vector_string_pass() {
        let mut mhmap = Vec::<String>::new();
        mhmap.push(String::from("foo"));
        mhmap.push(String::from("bar"));
        let result = get_runner_yaml();
        let desc = Deseriaizer::new(&result[INDEX_VECTOR_STRING]);
        let data = mhmap.try_to_vec().unwrap();
        let deserialize_vector = desc.deser(&mut data.as_slice()).unwrap();
        println!(
            "{}",
            serde_json::to_string_pretty(&desc.to_json(&deserialize_vector)).unwrap()
        );
    }

    #[test]
    fn test_vector_u32_pass() {
        let mut mhmap = Vec::<u32>::new();
        mhmap.push(1u32);
        mhmap.push(2u32);
        let result = get_runner_yaml();
        let desc = Deseriaizer::new(&result[INDEX_VECTOR_U32]);
        let data = mhmap.try_to_vec().unwrap();
        let deserialize_vector = desc.deser(&mut data.as_slice()).unwrap();
        println!("{:?}", deserialize_vector);
        println!(
            "{}",
            serde_json::to_string_pretty(&desc.to_json(&deserialize_vector)).unwrap()
        );
    }

    #[test]
    fn test_tuple_pass() {
        let mhmap = OfTuple("Foo".to_string(), 19u128);
        let result = get_runner_yaml();
        let desc = Deseriaizer::new(&result[INDEX_TUPLE_STRING_U128]);

        let data = mhmap.try_to_vec().unwrap();
        println!("{:?}", data);
        let deserialize_vector = desc.deser(&mut data.as_slice()).unwrap();
        println!("deser {:?}", deserialize_vector);
        println!("types{:?}", desc.schema().item_type_prefixes());
        println!(
            "{}",
            serde_json::to_string_pretty(&desc.to_json(&deserialize_vector)).unwrap()
        );
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
        let deserialize_vector = desc.deser(&mut data.as_slice()).unwrap();
        println!("deser {:?}", deserialize_vector);
        println!("types{:?}", desc.schema().item_type_prefixes());
        println!(
            "{}",
            serde_json::to_string_pretty(&desc.to_json(&deserialize_vector)).unwrap()
        );
    }

    #[test]
    fn pubkey_pass() {
        let result = get_runner_yaml();
        let desc = Deseriaizer::new(&result[INDEX_PUBLICKEY]);
        let pk = Pubkey::from_str("A94wMjV54C8f8wn7zL8TxNCdNiGoq7XSN7vWGrtd4vwU").unwrap();
        let pk_ser = pk.try_to_vec().unwrap();
        println!("{:?}", pk_ser);
        let deserialize_vector = desc.deser(&mut pk_ser.as_slice()).unwrap();
        println!("deser {:?}", deserialize_vector);
        println!("types{:?}", desc.schema().item_type_prefixes());
        println!(
            "{}",
            serde_json::to_string_pretty(&desc.to_json(&deserialize_vector)).unwrap()
        );
    }
}
