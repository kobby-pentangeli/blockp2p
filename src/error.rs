/// All error types
#[derive(Debug, thiserror::Error)]
pub enum BlockP2pError {
    /// Error from serializing hash compute data
    #[error("{0}")]
    SerializeHashError(String),
}
