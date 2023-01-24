use crate::{
    connection::routing::RoutingTable,
    crypto::{hash::Hash, signature::Signature},
    error::Error,
    event::Event,
    identity::{keys::Keys, Identity},
};
use bytes::Bytes;
use crossbeam_channel::Sender;
use message::Message;
use qp2p::{Connection as QuicConnection, Endpoint as QuicEndpoint};
use std::{
    collections::{hash_map::Entry, HashMap},
    net::SocketAddr,
};

/// Types of peer-to-peer messages
pub mod message;

// Time to live
const TTL: usize = 5;
const OUTBOX_COPIES: usize = 3;

/// Messaging functionality
#[derive(Debug, Default, Clone)]
pub struct Messaging {
    outbox: Vec<(Keys, Vec<u8>, usize)>,
    pending: Vec<(Bytes, u64)>,
}

impl Messaging {
    /// Creates a new `Messaging` type
    pub fn new() -> Self {
        Self {
            outbox: Default::default(),
            pending: Default::default(),
        }
    }

    /// Add an unsent message to the list of pending messages.
    pub fn handle_unsent_message(&mut self, msg: Bytes, tag: u64) -> Result<(), Error> {
        self.pending.push((msg, tag));
        Ok(())
    }
}
