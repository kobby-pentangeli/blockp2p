use crate::{crypto::hash::Hash, error::Error, messaging::message::Message};
use bytes::Bytes;
use qp2p::Connection as QuicConnection;
use routing::{RoutingTable, SharedRoutingTable};
use std::{collections::HashMap, net::SocketAddr};

/// Implements a routing table.
pub mod routing;

/// Map of all connections
pub type ConnectionMap = HashMap<SocketAddr, (Option<Hash>, ConnectionState)>;

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
    /// Hash of the node
    pub hash: Hash,
    /// Address of the node
    pub socket_addr: SocketAddr,
}

/// Manages the connection of a node
pub struct Connection {
    entries: ConnectionMap,
    active_connections: HashMap<Hash, SocketAddr>,
    routing_table: RoutingTable,
}

impl Connection {
    /// Creates a new `Connection`.
    pub fn new() -> Self {
        Self {
            entries: Default::default(),
            active_connections: Default::default(),
            routing_table: Default::default(),
        }
    }

    /// Update records in the routing table
    pub async fn update_routing_table(
        &mut self,
        self_id: Hash,
        peer_id: Hash,
        shared_table: SharedRoutingTable,
        quic: &mut QuicConnection,
    ) -> Result<(), Error> {
        let _ = shared_table
            .shared_entries()
            .keys()
            .into_iter()
            .map(|hash| {
                if !self.routing_table.has_node(hash) {
                    self.routing_table.add_new_node(hash);
                }
            })
            .collect::<Vec<_>>();

        let mut changed = false;
        let _ = self
            .routing_table
            .entries_mut()
            .iter_mut()
            .map(|(dest, (hop_to, count))| {
                if let Some(new_hop_count) = shared_table.shared_routing_info(dest) {
                    if new_hop_count + 1 < *count {
                        changed = true;
                        let _ = std::mem::replace(hop_to, peer_id);
                        let _ = std::mem::replace(count, new_hop_count + 1);
                    }
                }
            })
            .collect::<Vec<_>>();
        if changed {
            self.routing_table.increment_version();
            self.share_routing_table(quic, self_id).await?;
        }
        Ok(())
    }

    /// Share routing information with peers
    pub async fn share_routing_table(
        &mut self,
        quic: &mut QuicConnection,
        self_id: Hash,
    ) -> Result<(), Error> {
        let routing_table = self.routing_table();
        for socket_addr in self.active_connections().values() {
            let user_msg_bytes = (
                Bytes::from("Routing information"),
                Bytes::from(socket_addr.to_string()),
                Bytes::from(
                    bincode::serialize(&Message::RoutingTable {
                        shared_routing_table: routing_table.get_shared(),
                        source: self_id,
                    })
                    .map_err(|e| Error::BincodeSerializeError(e.to_string()))?,
                ),
            );
            quic.send(user_msg_bytes).await?;
        }
        Ok(())
    }

    /// Retrieves the routing table
    pub fn routing_table(&self) -> &RoutingTable {
        &self.routing_table
    }

    /// Returns the map of connections
    pub fn connections(&self) -> &ConnectionMap {
        &self.entries
    }

    /// Returns the map of active connections
    pub fn active_connections(&self) -> &HashMap<Hash, SocketAddr> {
        &self.active_connections
    }
}

impl Default for Connection {
    fn default() -> Self {
        Self::new()
    }
}
