use crate::crypto::{
    keys::{EncryptionPrivateKey, EncryptionPublicKey, SigningPrivateKey, SigningPublicKey},
    signature::{PrivateKey, PublicKey},
};

/// Implements the public identity of a node
pub mod public_id;

/// Identity of a p2p node
pub struct Identity {
    secret_key: PrivateKey,
    public_key: PublicKey,
    encryption_secret_key: EncryptionPrivateKey,
    encryption_public_key: EncryptionPublicKey,
    signing_secret_key: SigningPrivateKey,
    signing_public_key: SigningPublicKey,
}

impl Identity {
    /// Creates a new random `Identity`.
    pub fn new() -> Self {
        todo!()
    }
}
