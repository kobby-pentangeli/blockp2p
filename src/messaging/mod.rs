use crate::{
    connection::routing::RoutingTable,
    crypto::hash::Hash,
    error::Error,
    event::Event,
    identity::{keys::Keys, Identity},
};
use bytes::Bytes;
use crossbeam_channel::Sender;
use ed25519_dalek::Signature;
use ed25519_dalek::Verifier;
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

    /// Send an ordinary message to a peer
    pub fn send_message(&mut self, dst: &Keys, message: &[u8]) -> Result<(), Error> {
        todo!()
    }

    /// Send a message to a peer using public key encryption
    pub fn send_encrypted_message(&mut self, dst: &Keys, message: &[u8]) -> Result<(), Error> {
        todo!()
    }

    /// Send a message to a peer using authenticated encryption
    pub fn send_authenticated_message(
        &mut self,
        self_id: &Identity,
        dst: &Keys,
        message: &[u8],
    ) -> Result<(), Error> {
        todo!()
    }

    /// Sign a message and send it
    pub fn send_signed_message(
        &mut self,
        self_id: &Identity,
        dst: &Keys,
        message: &[u8],
    ) -> Result<(), Error> {
        todo!()
    }

    fn send_pending_messages(
        &mut self,
        active_connections: &[&SocketAddr],
        quic: &mut QuicConnection,
    ) -> Result<(), Error> {
        todo!()
    }

    /// Send agent message
    pub fn send_agent_message(
        &mut self,
        mut payload: Vec<(Keys, Vec<u8>)>,
        active_connections: &[&SocketAddr],
        first: bool,
        quic: &mut QuicConnection,
    ) -> Result<(), Error> {
        todo!()
    }

    /// Add an unsent message to the list of pending messages.
    pub fn handle_unsent_message(&mut self, msg: Bytes, tag: u64) -> Result<(), Error> {
        self.pending.push((msg, tag));
        Ok(())
    }

    /// Process message
    pub fn handle_agent_message(
        &mut self,
        self_id: &Identity,
        peer: &mut QuicEndpoint,
        mut payload: Vec<(Keys, Vec<u8>)>,
        active_connections: &[&SocketAddr],
        quic: &mut QuicConnection,
        tx: &Sender<Event>,
    ) -> Result<(), Error> {
        todo!()
    }

    fn handle_message(
        &mut self,
        peer: &mut QuicEndpoint,
        msg: Vec<u8>,
        self_id: &Identity,
        tx: &Sender<Event>,
    ) -> Result<(), Error> {
        match bincode::deserialize::<Message>(&msg) {
            Ok(Message::UserMessage(content)) => {
                log::trace!(
                    "Peer at {:?} sent: {:?}",
                    &peer.public_addr(),
                    &content[..4]
                );
                tx.send(Event::NewMessage(content))?;
                Ok(())
            }
            Ok(Message::EncryptedMessage(content)) => {
                log::trace!(
                    "Peer at {:?} sent an encrypted message: {:?}",
                    &peer.public_addr(),
                    &content[..4]
                );
                let decrypted_msg = self_id.decrypt_message(&content)?;
                tx.send(Event::NewMessage(decrypted_msg))?;
                Ok(())
            }
            Ok(Message::AuthenticatedMessage { message, sender }) => {
                log::warn!(
                    "Peer at {:?} sent an authenticated message: {:?}",
                    &peer.public_addr(),
                    &message[..4]
                );
                let verified_msg = self_id.verify_message(sender, &message)?;
                tx.send(Event::NewMessage(verified_msg))?;
                Ok(())
            }
            Ok(Message::SignedMessage {
                message,
                signature,
                sender,
            }) => {
                log::trace!(
                    "Peer at {:?} sent a signed message: {:?}",
                    &peer.public_addr(),
                    &message[..4]
                );
                if let Ok(signature) = Signature::from_bytes(&signature) {
                    if let Ok(()) = sender.signing_public_key.verify(&message, &signature) {
                        tx.send(Event::NewMessage(message))?;
                    } else {
                        log::error!("Message dropped; invalid signature!");
                    }
                }
                Ok(())
            }
            _ => {
                log::error!("Unexpected message!");
                Ok(())
            }
        }
    }
}
