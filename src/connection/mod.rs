use crate::{
    crypto::hash::Hash, Event, Message, PublicId, Result, RoutingTable, SharedRoutingTable,
};
use bytes::Bytes;
use connection_types::{ConnectionInfo, ConnectionMap, ConnectionState};
use crossbeam_channel::Sender;
use qp2p::{Connection as QuicConnection, Endpoint as QuicEndpoint};
use std::{collections::hash_map::Entry, net::SocketAddr};

/// Connection-related types
pub mod connection_types;
/// Implements a routing table.
pub mod routing;

pub(super) const MAX_CONNECTION_LEN: usize = 5;

/// Manages the connection of a node
pub struct Connection {
    entries: ConnectionMap,
    routing_table: RoutingTable,
    is_bootstrapped: bool,
}

impl Connection {
    /// Creates a new `Connection`.
    pub fn new() -> Self {
        Self {
            entries: Default::default(),
            routing_table: Default::default(),
            is_bootstrapped: false,
        }
    }

    /// Update records in the routing table
    pub async fn update_routing_table(
        &mut self,
        self_id: &Hash,
        peer_id: &Hash,
        shared_table: SharedRoutingTable,
        quic: &mut QuicConnection,
    ) -> Result<()> {
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
                        let _ = std::mem::replace(hop_to, *peer_id);
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
        self_id: &Hash,
    ) -> Result<()> {
        let routing_table = self.routing_table();
        for socket_addr in self.active_connections() {
            let user_msg_bytes = (
                Bytes::from("Routing information"),
                Bytes::from(socket_addr.to_string()),
                Bytes::from(bincode::serialize(&Message::RoutingTable {
                    shared_routing_table: routing_table.get_shared(),
                    source: *self_id,
                })?),
            );
            quic.send(user_msg_bytes).await?;
        }
        Ok(())
    }

    /// Handle a node-identification message from a peer.
    /// Returns `true` if an agent should be deployed.
    pub async fn handle_peer_identification(
        &mut self,
        self_id: &PublicId,
        peer: &mut QuicEndpoint,
        peer_id: &PublicId,
        sender: &Sender<Event>,
        quic: &mut QuicConnection,
    ) -> Result<bool> {
        let peer_addr = peer.local_addr();
        log::debug!(
            "Peer at {:?} has identified itself as {:?}",
            &peer_addr,
            &peer_id
        );
        let mut connected = false;
        if let Entry::Occupied(mut entry) = self.entries.entry(peer_addr) {
            let (id, state) = entry.get_mut();
            if id.is_none() {
                let _ = std::mem::replace(id, Some(*peer_id));
                let _ = std::mem::replace(state, ConnectionState::Connected);

                sender.send(Event::ConnectedTo(*peer_id))?;
                self.routing_table.add_direct_connection(&peer_id.node_id);
                self.routing_table.increment_version();
                connected = true;

                log::debug!("Successfully connected with peer at {:?}", peer_addr);
                log::debug!("Our connections: {:?}", &self.entries);
            }
        }
        if connected && !self.is_bootstrapped() {
            self.set_bootstrapped();
            self.share_routing_table(quic, &self_id.node_id).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Connect to a peer.
    /// Used when both a peer's public identity and socket address are known.
    pub async fn connect_to(
        &mut self,
        info: &ConnectionInfo,
        quic: &mut QuicEndpoint,
    ) -> Result<()> {
        log::trace!("Connecting to: {:?}", info);
        let _ = self.entries.insert(
            info.socket_addr,
            (Some(info.public_id), ConnectionState::Connecting),
        );
        let _ = quic.connect_to(&info.socket_addr).await?;
        Ok(())
    }

    /// Handle a successful incoming connection.
    /// Returns `true` if an agent is to be deployed.
    pub async fn handle_successful_connection(
        &mut self,
        self_id: &PublicId,
        peer: &mut QuicEndpoint,
        sender: &Sender<Event>,
        quic: &mut QuicConnection,
    ) -> Result<bool> {
        let peer_addr = peer.local_addr();
        let mut connected = false;
        if let Some((public_identity, state)) = self.entries.get_mut(&peer_addr) {
            let user_msg_bytes = (
                Bytes::from("Public identity"),
                Bytes::from(peer_addr.to_string()),
                Bytes::from(bincode::serialize(&Message::Identification(*self_id))?),
            );
            quic.send(user_msg_bytes).await?;

            if let Some(id) = public_identity {
                let _ = std::mem::replace(state, ConnectionState::Connected);

                sender.send(Event::ConnectedTo(*id))?;

                self.routing_table.add_direct_connection(&id.node_id);
                self.routing_table.increment_version();
                connected = true;
                log::debug!("Successfully connected with peer at {:?}", peer_addr);
                log::debug!("Our connections: {:?}", &self.entries);
            } else {
                log::debug!("Waiting to identify peer at {:?}", peer_addr);
            }
        } else if self.entries.len() == MAX_CONNECTION_LEN {
            let connections = self.entries.keys().copied().collect::<Vec<SocketAddr>>();
            log::warn!("Too many connections! Disconnecting from {:?}", &peer_addr);
            let user_msg_bytes = (
                Bytes::from("Contacts"),
                Bytes::from(peer_addr.to_string()),
                Bytes::from(bincode::serialize(&Message::Contacts(connections))?),
            );
            quic.send_with(user_msg_bytes, 1).await?;
            return Ok(false);
        } else {
            let _ = self
                .entries
                .insert(peer_addr, (None, ConnectionState::Incoming));
            let user_msg_bytes = (
                Bytes::from("Public identity"),
                Bytes::from(peer_addr.to_string()),
                Bytes::from(bincode::serialize(&Message::Identification(*self_id))?),
            );
            quic.send(user_msg_bytes).await?;
        }
        if connected && !self.is_bootstrapped() {
            self.set_bootstrapped();
            self.share_routing_table(quic, &self_id.node_id).await?;
            Ok(true)
        } else {
            log::trace!("Our connections: {:?}", &self.entries);
            Ok(false)
        }
    }

    /// Disseminate appropriate information on connection failure.
    pub fn handle_connection_failure(
        &mut self,
        quic: &mut QuicEndpoint,
        err_msg: &str,
    ) -> Result<()> {
        let peer_addr = quic.local_addr();
        log::info!(
            "Lost connection with peer at {:?} due to {}",
            &peer_addr,
            err_msg
        );
        if let Some((id, _state)) = self.entries.remove(&peer_addr) {
            log::info!(
                "Disconnected from peer at {:?} with ID {:?}",
                &peer_addr,
                id
            );
        } else {
            log::warn!(
                "Connection with peer at {:?} was dropped before the operation",
                &peer_addr
            );
        }
        Ok(())
    }

    /// Bootstrap to the network using all our contacts
    pub async fn bootstrap(&mut self, nodes: &[SocketAddr], quic: &mut QuicEndpoint) -> Result<()> {
        for node in nodes {
            if self.entries.len() == MAX_CONNECTION_LEN {
                break;
            }
            if !self.entries.contains_key(node) {
                self.bootstrap_with(node, quic).await?;
            }
        }
        Ok(())
    }

    /// Bootstrap with a peer.
    /// Used when only one peer's socket address is known.
    pub async fn bootstrap_with(
        &mut self,
        socket_addr: &SocketAddr,
        quic: &mut QuicEndpoint,
    ) -> Result<()> {
        let _ = self
            .entries
            .insert(*socket_addr, (None, ConnectionState::Connecting));
        let _ = quic.connect_to(socket_addr).await?;
        Ok(())
    }

    /// Checks if a node is bootstrapped to the network
    pub fn is_bootstrapped(&self) -> bool {
        self.is_bootstrapped
    }

    /// Change bootstrap status to true when a node is bootstrapped to the network
    pub fn set_bootstrapped(&mut self) {
        self.is_bootstrapped = true;
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
    pub fn active_connections(&self) -> Vec<&SocketAddr> {
        self.entries
            .iter()
            .filter(|(_, (_, state))| state == &ConnectionState::Connected)
            .map(|(socket_addr, _)| socket_addr)
            .collect::<Vec<_>>()
    }
}

impl Default for Connection {
    fn default() -> Self {
        Self::new()
    }
}
