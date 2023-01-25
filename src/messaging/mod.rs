use crate::{Event, Identity, Keys, Message, Result};
use bytes::Bytes;
use crossbeam_channel::Sender;
use ed25519_dalek::{Signature, Verifier};
use qp2p::{Connection as QuicConnection, Endpoint as QuicEndpoint};
use rand::Rng;
use std::net::SocketAddr;

/// Types of peer-to-peer messages
pub mod message;

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
    pub fn send_message(&mut self, dst: &Keys, message: &[u8]) -> Result<()> {
        self.outbox.push((
            *dst,
            bincode::serialize(&Message::UserMessage(message.to_vec()))?,
            OUTBOX_COPIES,
        ));
        Ok(())
    }

    /// Send a message to a peer using public key encryption
    pub fn send_encrypted_message(&mut self, dst: &Keys, message: &[u8]) -> Result<()> {
        let cypher_text = dst.public_key.0.encrypt(message);
        let cypher_bytes = bincode::serialize(&cypher_text)?;
        self.outbox.push((
            *dst,
            bincode::serialize(&Message::EncryptedMessage(cypher_bytes))?,
            OUTBOX_COPIES,
        ));
        Ok(())
    }

    /// Send a message to a peer using authenticated encryption
    pub fn send_authenticated_message(
        &mut self,
        self_id: &Identity,
        dst: &Keys,
        message: &[u8],
    ) -> Result<()> {
        let cypher_bytes = self_id.authenticate_message(dst, message);
        self.outbox.push((
            *dst,
            bincode::serialize(&Message::AuthenticatedMessage {
                message: cypher_bytes,
                sender: self_id.keys(),
            })?,
            OUTBOX_COPIES,
        ));
        Ok(())
    }

    /// Sign a message and send it
    pub fn send_signed_message(
        &mut self,
        self_id: &Identity,
        dst: &Keys,
        message: &[u8],
    ) -> Result<()> {
        let signature = self_id.sign_message(message);
        self.outbox.push((
            *dst,
            bincode::serialize(&Message::SignedMessage {
                message: message.to_vec(),
                signature: signature.as_bytes().to_vec(),
                sender: self_id.keys(),
            })?,
            OUTBOX_COPIES,
        ));
        Ok(())
    }

    /// Send agent message
    pub async fn send_agent_message(
        &mut self,
        mut payload: Vec<(Keys, Vec<u8>)>,
        active_connections: &[&SocketAddr],
        first: bool,
        quic: &mut QuicConnection,
    ) -> Result<()> {
        if active_connections.is_empty() {
            log::error!("No active connections!");
            return Ok(());
        }
        self.send_pending_messages(active_connections, quic).await?;
        let _ = self
            .outbox
            .iter_mut()
            .map(|(a, b, c)| {
                payload.push((*a, b.to_vec()));
                let _ = std::mem::replace(c, *c - 1);
            })
            .collect::<Vec<()>>();

        self.outbox.retain(|(_, _, count)| *count > 0);
        let mut rng = rand::thread_rng();
        let rand_val = rng.gen_range(0..active_connections.len());
        if let Some(addr) = active_connections.get(rand_val) {
            let user_msg_bytes = (
                Bytes::from("Agent message"),
                Bytes::from(addr.to_string()),
                Bytes::from(bincode::serialize(&Message::AgentMessage { payload })?),
            );
            quic.send(user_msg_bytes).await?;
            if first {
                log::debug!("Agent deployed to: {:?}", addr);
            }
        }
        Ok(())
    }

    async fn send_pending_messages(
        &mut self,
        active_connections: &[&SocketAddr],
        quic: &mut QuicConnection,
    ) -> Result<()> {
        if self.pending.is_empty() || active_connections.is_empty() {
            log::error!("No pending messages or active connections!");
            return Ok(());
        }
        let conn_len = active_connections.len();
        let mut rng = rand::thread_rng();
        while let Some((msg, tag)) = self.pending.pop() {
            let rand_val = rng.gen_range(0..conn_len);
            if let Some(addr) = active_connections.get(rand_val) {
                let user_msg_bytes = (
                    Bytes::from(tag.to_string()),
                    Bytes::from(addr.to_string()),
                    msg,
                );
                quic.send(user_msg_bytes).await?;
            }
        }
        Ok(())
    }

    /// Add an unsent message to the list of pending messages.
    pub fn handle_unsent_message(&mut self, msg: Bytes, tag: u64) -> Result<()> {
        self.pending.push((msg, tag));
        Ok(())
    }

    /// Process message
    pub async fn handle_agent_message(
        &mut self,
        self_id: &Identity,
        peer: &mut QuicEndpoint,
        mut payload: Vec<(Keys, Vec<u8>)>,
        active_connections: &[&SocketAddr],
        quic: &mut QuicConnection,
        tx: &Sender<Event>,
    ) -> Result<()> {
        let self_pub_keys = self_id.keys();
        let mut forward = vec![];
        while let Some((target_keys, msg)) = payload.pop() {
            if target_keys == self_pub_keys {
                self.handle_message(peer, msg, self_id, tx)?;
            } else {
                forward.push((target_keys, msg));
            }
        }
        self.send_agent_message(forward, active_connections, false, quic)
            .await
    }

    fn handle_message(
        &mut self,
        peer: &mut QuicEndpoint,
        msg: Vec<u8>,
        self_id: &Identity,
        tx: &Sender<Event>,
    ) -> Result<()> {
        match bincode::deserialize::<Message>(&msg) {
            Ok(Message::UserMessage(content)) => {
                log::trace!("Peer at {:?} sent: {:?}", &peer.local_addr(), &content[..4]);
                tx.send(Event::NewMessage(content))?;
                Ok(())
            }
            Ok(Message::EncryptedMessage(content)) => {
                log::trace!(
                    "Peer at {:?} sent an encrypted message: {:?}",
                    &peer.local_addr(),
                    &content[..4]
                );
                let decrypted_msg = self_id.decrypt_message(&content)?;
                tx.send(Event::NewMessage(decrypted_msg))?;
                Ok(())
            }
            Ok(Message::AuthenticatedMessage { message, sender }) => {
                log::warn!(
                    "Peer at {:?} sent an authenticated message: {:?}",
                    &peer.local_addr(),
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
                    &peer.local_addr(),
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
