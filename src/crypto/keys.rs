/// A Diffie-Hellman public key
pub type EncryptionPublicKey = x25519_dalek::PublicKey;

/// A Diffie-Hellman secret key
pub type EncryptionPrivateKey = x25519_dalek::StaticSecret;

/// An Ed25519 public key
pub type SigningPublicKey = ed25519_dalek::PublicKey;

/// An EdDSA secret key
pub type SigningPrivateKey = ed25519_dalek::SecretKey;

/// An ed25519 keypair
pub type EdKeypair = ed25519_dalek::Keypair;
