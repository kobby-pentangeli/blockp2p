use crate::event::Event;

/// All error types
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Errors associated with BLS private keys, public keys, and signatures
    #[error("{0}")]
    BlsThresholdCryptoError(blsttc::Error),

    /// Errors that can occur when sending messages
    #[error("{0}")]
    QuicSendError(qp2p::SendError),

    /// Errors that can cause connection loss
    #[error("{0}")]
    QuicConnectionError(qp2p::ConnectionError),

    /// Message not sent because the channel is disconnected
    #[error("{0}")]
    CrossbeamSendError(crossbeam_channel::SendError<Event>),

    /// Serialization errors
    #[error("{0}")]
    BincodeSerializeError(bincode::Error),

    /// No routing information found for this node
    #[error("No routing information found for this node")]
    NoRoutingInformation,
}

impl From<blsttc::Error> for Error {
    fn from(value: blsttc::Error) -> Self {
        Error::BlsThresholdCryptoError(value)
    }
}

impl From<bincode::Error> for Error {
    fn from(value: bincode::Error) -> Self {
        Error::BincodeSerializeError(value)
    }
}

impl From<qp2p::SendError> for Error {
    fn from(value: qp2p::SendError) -> Self {
        Error::QuicSendError(value)
    }
}

impl From<qp2p::ConnectionError> for Error {
    fn from(value: qp2p::ConnectionError) -> Self {
        Error::QuicConnectionError(value)
    }
}

impl From<crossbeam_channel::SendError<Event>> for Error {
    fn from(value: crossbeam_channel::SendError<Event>) -> Self {
        Error::CrossbeamSendError(value)
    }
}
