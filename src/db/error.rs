#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("failed to comunicate database")]
    TransactionError(#[from] worker::Error),
    #[error("failed to found target {0}")]
    NotFound(String),
}
