use crate::{
    connection::connection_types::{ConnectionInfo, ConnectionMap},
    Config, Connection, Event, Identity, Message, Messaging, PublicId, Result,
};
use bytes::Bytes;
use crossbeam_channel::{Receiver, Select, Sender};
use qp2p::{Connection as QuicConnection, Endpoint as QuicEndpoint};
use std::net::SocketAddr;

/// Representation of a peer-to-peer node
pub struct Node {
    config: Config,
    identity: Identity,
    connection: Connection,
    messaging: Messaging,
    channel_tx: Sender<Event>,
    channel_rx: Receiver<Event>,
}

impl Node {
    /// Creates a new `Node`.
    pub fn new() -> Result<(Self, Receiver<Event>)> {
        Self::with_config(Config::default())
    }

    /// Creates a new `Node` with specified configuration.
    pub fn with_config(config: Config) -> Result<(Self, Receiver<Event>)> {
        let (channel_tx, channel_rx) = crossbeam_channel::unbounded::<Event>();
        Ok((
            Self {
                config,
                identity: Identity::new(),
                connection: Connection::new(),
                messaging: Messaging::new(),
                channel_tx,
                channel_rx: channel_rx.clone(),
            },
            channel_rx,
        ))
    }

    /// Fetch map of connections.
    pub fn connections(&self) -> &ConnectionMap {
        self.connection.connections()
    }

    /// Retrieves the connection information
    pub fn connection_info(&self, quic: &QuicEndpoint) -> ConnectionInfo {
        ConnectionInfo {
            public_id: self.identity.public_id(),
            socket_addr: quic.local_addr(),
        }
    }

    /// Bootstrap to the network using all contacts
    pub async fn bootstrap(&mut self, quic: &mut QuicEndpoint) -> Result<()> {
        let nodes = self
            .config
            .bootstrap_nodes()
            .cloned()
            .collect::<Vec<SocketAddr>>();
        self.connection.bootstrap(&nodes, quic).await
    }

    /// Bootstrap with a peer.
    /// Used when only the peer's socket address is known.
    pub async fn bootstrap_with(
        &mut self,
        addr: SocketAddr,
        quic: &mut QuicEndpoint,
    ) -> Result<()> {
        log::trace!("Bootstrapping with peer at: {:?}", &addr);
        self.connection.bootstrap_with(&addr, quic).await
    }

    /// Connect to a peer.
    /// Used when both a peer's socket address and public key are known.
    pub async fn connect_to(
        &mut self,
        info: &ConnectionInfo,
        quic: &mut QuicEndpoint,
    ) -> Result<()> {
        log::trace!("Connecting to peer at: {:?}", &info);
        self.connection.connect_to(info, quic).await
    }

    /// Register a selector for events
    pub fn register_selector<'a>(&'a mut self, selector: &mut Select<'a>) -> usize {
        selector.recv(&self.channel_rx)
    }

    /// Send a message to a peer
    pub fn send_message(&mut self, dst: &PublicId, msg: &[u8]) -> Result<()> {
        log::trace!("Sending message to {:?}", dst);
        self.messaging.send_message(dst, msg)
    }

    /// Send a message to a peer using public-key encryption
    pub fn send_encrypted_message(&mut self, dst: &PublicId, msg: &[u8]) -> Result<()> {
        log::trace!("Sending encrypted message to {:?}", dst);
        self.messaging.send_encrypted_message(dst, msg)
    }

    /// Send a message to a peer using authenticated encryption
    pub fn send_authenticated_message(&mut self, dst: &PublicId, msg: &[u8]) -> Result<()> {
        log::trace!("Sending authenticated message to {:?}", dst);
        self.messaging
            .send_authenticated_message(&self.identity, dst, msg)
    }

    /// Send a message along with a signature
    pub fn send_signed_message(&mut self, dst: &PublicId, msg: &[u8]) -> Result<()> {
        log::trace!("Sending signed message to {:?}", dst);
        self.messaging.send_signed_message(&self.identity, dst, msg)
    }

    /// Handle an incoming node event
    pub async fn handle_incoming_event(&mut self) -> Result<()> {
        todo!()
    }

    async fn handle_incoming_message(
        &mut self,
        peer: &mut QuicEndpoint,
        msg: &Bytes,
        quic: &mut QuicConnection,
    ) -> Result<()> {
        match bincode::deserialize::<Message>(msg)? {
            Message::Identification(public_id) => {
                let deploy_agent = self
                    .connection
                    .handle_peer_identification(
                        &self.identity.public_id(),
                        peer,
                        &public_id,
                        &self.channel_tx,
                        quic,
                    )
                    .await?;
                if deploy_agent {
                    self.messaging
                        .send_agent_message(
                            Vec::new(),
                            &self.connection.active_connections(),
                            false,
                            quic,
                        )
                        .await?;
                }
                Ok(())
            }
            Message::AgentMessage { payload } => {
                log::trace!("Got a message from an agent");
                self.messaging
                    .handle_agent_message(
                        &self.identity,
                        peer,
                        payload,
                        &self.connection.active_connections(),
                        quic,
                        &self.channel_tx,
                    )
                    .await?;
                Ok(())
            }
            Message::Contacts(contacts) => {
                self.connection.bootstrap(&contacts, peer).await?;
                Ok(())
            }
            message => {
                log::error!("Peer {:?} sent us: {:?}", peer.local_addr(), &message);
                Ok(())
            }
        }
    }
}
