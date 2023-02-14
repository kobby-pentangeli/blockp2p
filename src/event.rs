use crate::PublicId;
use qp2p::Endpoint as QuicEndpoint;

/// Types of peer-to-peer events
#[derive(Debug)]
pub enum Event {
    /// Events regarding the receipt of a new message
    NewMessage(Vec<u8>),

    /// Events regarding the sending of a new user message
    SentUserMessage {
        /// Intended recipient
        peer: PublicId,
        /// Payload
        message: Vec<u8>,
    },

    /// Unsent user messages
    UnsentUserMessage {
        /// Payload
        message: Vec<u8>,
        /// Tag
        token: u64,
    },

    /// Events regarding a successful connection
    ConnectedTo(PublicId),

    /// Events regarding a failed connection
    ConnectionFailure {
        /// Desired endpoint
        peer: QuicEndpoint,
        /// Error
        err: String,
    },
}
