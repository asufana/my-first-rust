use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("ValidationError: {0}")]
    ValidationError(String),
}
