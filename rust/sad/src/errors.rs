use thiserror::Error;

#[derive(Error, Debug)]
pub enum SadTreeError {
    #[error("Don't know type {0}")]
    UnknownType(String),
    #[error("Expected YAML HashMap")]
    ExpectedHashMap,
    #[error("Expected YAML Array")]
    ExpectedArray,
    #[error("Expected YAML HashMap or Array")]
    ExpectedHashMapOrArray,
    #[error("Expected HashMap fields")]
    ExpectedHashMapFields,
}
