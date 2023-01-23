/// All error types
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Errors associated with BLS public keys
    #[error("{0}")]
    BlsPublicKeyError(String),

    /// Errors associated with BLS private keys
    #[error("{0}")]
    BlsPrivateKeyError(String),

    /// Errors associated with BLS signatures
    #[error("{0}")]
    BlsSignatureError(String),
}
