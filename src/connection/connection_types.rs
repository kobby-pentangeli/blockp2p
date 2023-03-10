use crate::PublicId;
use std::{collections::HashMap, net::SocketAddr};

/// Map of all connections
pub type ConnectionMap = HashMap<SocketAddr, (Option<PublicId>, ConnectionState)>;

/// Connection state
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ConnectionState {
    /// Initialized outgoing connection
    Connecting,
    /// Incoming connection from a peer
    Incoming,
    /// Connection established
    Connected,
}

/// Connection information for a node
#[derive(Debug, Clone, Copy)]
pub struct ConnectionInfo {
    /// Public identity of the node
    pub public_id: PublicId,
    /// Address of the node
    pub socket_addr: SocketAddr,
}
