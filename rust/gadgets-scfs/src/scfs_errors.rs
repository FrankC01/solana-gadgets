//! @brief Error sets for Scfs

use thiserror::Error;
#[derive(Error, Debug)]
pub enum ScfsError {
    #[error("Criteria must have at least Some feature keys")]
    NoCriteriaFeaturesError,
    #[error("Error validatinig {ctype} invalid {ctype:?}")]
    UnrecognizedCriteriaTypeError {
        ctype: &'static str,
        element: Vec<String>,
    },
}

pub type ScfsResult<T> = std::result::Result<T, ScfsError>;
