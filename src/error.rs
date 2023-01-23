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

    /// Errors that can occur when sending messages
    #[error("{0}")]
    QuicSendError(qp2p::SendError),

    /// Serialization errors
    #[error("{0}")]
    BincodeSerializeError(String),
}

impl From<qp2p::SendError> for Error {
    fn from(value: qp2p::SendError) -> Self {
        Error::QuicSendError(value)
    }
}
