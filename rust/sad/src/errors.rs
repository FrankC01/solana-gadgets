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
    #[error("Expected Vec contains")]
    ExpectedVecContains,
    #[error("Expected Tuple fields")]
    ExpectedTupleFields,
    #[error("Expected CStruct fields")]
    ExpectedCStructFields,
}

#[derive(Error, Debug)]
pub enum SadAccountErrorType {
    #[error("Failed getting Account from cluster")]
    FailedAccountGet,
    #[error("Failed getting Program Accounts from cluster")]
    FailedProgramAccountGet,
}

pub type SadTreeResult<T> = std::result::Result<T, SadTreeError>;
pub type SadAccountResult<T> = std::result::Result<T, SadAccountErrorType>;