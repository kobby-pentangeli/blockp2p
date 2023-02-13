use crate::crypto::{hash::Hash, signature::PublicKey, EncryptionPublicKey, SigningPublicKey};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Represents the various public keys belonging to a node
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PublicId {
    /// Node id
    pub node_id: Hash,
    /// Public key for asymmetric encryption
    pub public_key: PublicKey,
    /// Public key for symmetric encryption
    pub encryption_public_key: EncryptionPublicKey,
    /// Public key for signing messages
    pub signing_public_key: SigningPublicKey,
}

impl Serialize for PublicId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (
            &self.node_id,
            &self.public_key,
            &self.encryption_public_key.as_bytes(),
            &self.signing_public_key.as_bytes(),
        )
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PublicId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (node_id, public_key, encr_bytes, sign_bytes): (Hash, PublicKey, [u8; 32], [u8; 32]) =
            Deserialize::deserialize(deserializer)?;
        Ok(Self {
            node_id,
            public_key,
            encryption_public_key: EncryptionPublicKey::from(encr_bytes),
            signing_public_key: SigningPublicKey::from_bytes(&sign_bytes)
                .expect("Failed to construct the ed25519_dalek::PublicKey from bytes"),
        })
    }
}
