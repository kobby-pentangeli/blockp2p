/// All error types
#[derive(Debug, thiserror::Error)]
pub enum BlockP2pError {
    /// Error from serializing hash compute data
    #[error("{0}")]
    SerializeHashError(String),

    /// Errors associated with BLS public key
    #[error("{0}")]
    BlsPublicKeyError(String),

    /// Errors associated with BLS private key
    #[error("{0}")]
    BlsPrivateKeyError(String),
}
