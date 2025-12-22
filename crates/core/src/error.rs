use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("RPC error: {0}")]
    Rpc(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Kafka error: {0}")]
    Kafka(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Transaction failed: {0}")]
    TransactionFailed(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Internal(err.to_string())
    }
}
