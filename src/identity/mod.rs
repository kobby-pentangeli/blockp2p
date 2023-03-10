use crate::{
    crypto::{
        hash::Hash,
        signature::{PrivateKey, PublicKey, Signature},
        EncryptionPublicKey, EncryptionSecretKey, SigningPublicKey, SigningSecretKey,
    },
    PublicId, Result,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// The various public keys belonging to a node
pub mod public_id;

/// Identity of a p2p node
pub struct Identity {
    node_id: Hash,
    secret_key: PrivateKey,
    public_key: PublicKey,
    encryption_secret_key: EncryptionSecretKey,
    encryption_public_key: EncryptionPublicKey,
    signing_secret_key: SigningSecretKey,
    signing_public_key: SigningPublicKey,
}

impl Default for Identity {
    fn default() -> Self {
        Self::new()
    }
}

impl Identity {
    /// Creates a new random `Identity`.
    pub fn new() -> Self {
        todo!()
    }

    /// Verify that a message was sent from peer to `Self`,
    /// using authenticated encryption.
    pub fn verify_message(&self, _peer_id: PublicId, _msg: &[u8]) -> Result<Vec<u8>> {
        todo!()
    }

    /// Encrypt a message using authenticated encryption
    pub fn authenticate_message(&self, _peer_id: &PublicId, _msg: &[u8]) -> Vec<u8> {
        todo!()
    }

    /// Sign a given message
    pub fn sign_message(&self, _msg: &[u8]) -> Signature {
        todo!()
    }

    /// Verify a message's signature
    pub fn verify_signature(&self, _msg: &[u8], _sig: &Signature) -> Result<()> {
        todo!()
    }

    /// Returns the public key of this node.
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    /// Returns the public encryption key for encryption
    pub fn encryption_public_key(&self) -> &EncryptionPublicKey {
        &self.encryption_public_key
    }

    /// Get the set of public keys for this node.
    pub fn public_id(&self) -> PublicId {
        PublicId {
            node_id: self.node_id,
            public_key: self.public_key,
            encryption_public_key: self.encryption_public_key,
            signing_public_key: self.signing_public_key,
        }
    }

    /// Encode a node's identity into a string.
    pub fn encode_id(&self) -> Result<String> {
        todo!()
    }

    /// Decode a node's identity from a string.
    pub fn decode_id(_encoded_id: &str) -> Result<Self> {
        todo!()
    }

    /// Encrypt a message using the node's public key
    pub fn encrypt_message(&self, _data: &[u8]) -> Result<Vec<u8>> {
        todo!()
    }

    /// Decrypt a message using the node's secret key
    pub fn decrypt_message(&self, _data: &[u8]) -> Result<Vec<u8>> {
        todo!()
    }
}

impl Serialize for Identity {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (
            &self.secret_key,
            &self.public_key,
            &self.encryption_secret_key.to_bytes(),
            &self.encryption_public_key.as_bytes(),
            &self.signing_secret_key.to_bytes(),
            &self.signing_public_key.to_bytes(),
        )
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Identity {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (
            node_id,
            secret_key,
            public_key,
            encryption_secret_key_bytes,
            encryption_public_key_bytes,
            signing_secret_key_bytes,
            signing_public_key_bytes,
        ): (
            Hash,
            PrivateKey,
            PublicKey,
            [u8; 32],
            [u8; 32],
            [u8; 32],
            [u8; 32],
        ) = Deserialize::deserialize(deserializer)?;
        Ok(Identity {
            node_id,
            secret_key,
            public_key,
            encryption_secret_key: EncryptionSecretKey::from(encryption_secret_key_bytes),
            encryption_public_key: EncryptionPublicKey::from(encryption_public_key_bytes),
            signing_public_key: SigningPublicKey::from_bytes(&signing_public_key_bytes)
                .expect("Failed to create an `ed25519_dalek` public key"),
            signing_secret_key: SigningSecretKey::from_bytes(&signing_secret_key_bytes)
                .expect("Failed to create an `ed25519_dalek` secret key"),
        })
    }
}
