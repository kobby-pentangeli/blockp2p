use crate::{
    connection::utils::{ConnectionInfo, ConnectionMap},
    Config, Connection, Event, Identity, Messaging, Result,
};
use crossbeam_channel::{Receiver, Select, Sender};
use qp2p::Endpoint as QuicEndpoint;
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

    /// Handle an incoming node event
    pub fn handle_incoming_event(&mut self) -> Result<()> {
        todo!()
    }

    /// Fetch map of connections.
    pub fn connections(&self) -> &ConnectionMap {
        self.connection.connections()
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
}
