use crate::PublicId;

/// Types of peer-to-peer events
#[derive(Debug, PartialEq)]
pub enum Event {
    /// Events regarding the receipt of a new message
    NewMessage(Vec<u8>),

    /// Events regarding a successful connection
    ConnectedTo(PublicId),
}
