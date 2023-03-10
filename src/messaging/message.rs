use crate::{crypto::hash::Hash, PublicId, SharedRoutingTable};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// Types of peer-to-peer messages
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Message {
    /// Ordinary user message
    UserMessage(Vec<u8>),

    /// Message encrypted using a public key
    EncryptedMessage(Vec<u8>),

    /// Messaged encoded using authenticated encryption
    AuthenticatedMessage {
        /// Message
        message: Vec<u8>,
        /// Public identity of the sender
        sender: PublicId,
    },

    /// A signed message
    SignedMessage {
        /// Message
        message: Vec<u8>,
        /// Signature
        signature: Vec<u8>,
        /// Public identity of the sender
        sender: PublicId,
    },

    /// Message regarding the identification of a node
    Identification(PublicId),

    /// Message from contacts
    Contacts(Vec<SocketAddr>),

    /// Message from an agent
    AgentMessage {
        /// Agent payload
        payload: Vec<(PublicId, Vec<u8>)>,
    },

    /// Routing information
    RoutingTable {
        /// Shared routing information
        shared_routing_table: SharedRoutingTable,
        /// Source of the message
        source: Hash,
    },
}
