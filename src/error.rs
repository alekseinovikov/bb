#[derive(Debug, thiserror::Error)]
pub enum BbError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("invalid state: {0}")]
    InvalidState(String),
}
