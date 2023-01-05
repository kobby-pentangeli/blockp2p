use crate::crypto::{
    keys::{EncryptionPublicKey, SigningPublicKey},
    signature::PublicKey,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Represents the public identity of a node
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PublicId {
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
        let (public_key, encr_bytes, sign_bytes): (PublicKey, [u8; 32], [u8; 32]) =
            Deserialize::deserialize(deserializer)?;
        Ok(Self {
            public_key,
            encryption_public_key: EncryptionPublicKey::from(encr_bytes),
            signing_public_key: SigningPublicKey::from_bytes(&sign_bytes)
                .expect("Failed to construct the ed25519_dalek::PublicKey from bytes"),
        })
    }
}
