//! @brief Error sets for Scfs

use thiserror::Error;
#[derive(Error, Debug)]
pub enum ScfsError {
    #[error("Criteria must have at least Some feature keys")]
    NoCriteriaFeaturesError,
    #[error("Criteria contains unrecognized {ctype} types {:?}")]
    UnrecognizedCriteriaTypeError {
        ctype: &'static str,
        bad: Vec<String>,
    },
}

pub type ScfsResult<T> = std::result::Result<T, ScfsError>;
