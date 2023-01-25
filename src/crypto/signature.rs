use crate::Result;
use blsttc::{serde_impl::SerdeSecret, PK_SIZE, SIG_SIZE, SK_SIZE};
use serde::{Deserialize, Serialize};

/// A `blsttc` signature
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Signature(pub blsttc::Signature);

impl Signature {
    /// Creates a new `Signature` from a `blsttc` signature
    pub fn new(sig: blsttc::Signature) -> Self {
        Self(sig)
    }

    /// Sign a message
    pub fn sign<T: AsRef<[u8]>>(private_key: &PrivateKey, data: T) -> Self {
        Self::new(private_key.0.sign(data))
    }

    /// Creates a `Signature` from bytes
    pub fn from_bytes(data: [u8; SIG_SIZE]) -> Result<Self> {
        let sig = blsttc::Signature::from_bytes(data)?;
        Ok(Self(sig))
    }

    /// Convert a `Signature` to bytes
    pub fn as_bytes(&self) -> [u8; SIG_SIZE] {
        self.0.to_bytes()
    }

    /// Verify a message
    pub fn verify<T: AsRef<[u8]>>(&self, pub_key: &PublicKey, data: T) -> bool {
        pub_key.0.verify(&self.0, data)
    }
}

/// A `blsttc` public key
#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub struct PublicKey(pub blsttc::PublicKey);

impl PublicKey {
    /// Generates a `PublicKey` from bytes
    pub fn from_bytes(data: [u8; PK_SIZE]) -> Result<Self> {
        let pk = blsttc::PublicKey::from_bytes(data)?;
        Ok(Self(pk))
    }

    /// Convert a `PublicKey` to bytes
    pub fn as_bytes(&self) -> [u8; PK_SIZE] {
        self.0.to_bytes()
    }
}

/// A `blsttc` private key
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivateKey(pub SerdeSecret<blsttc::SecretKey>);

impl PrivateKey {
    /// Generates a `PrivateKey` from bytes
    pub fn from_bytes(data: [u8; SK_SIZE]) -> Result<Self> {
        let sk = blsttc::SecretKey::from_bytes(data)?;
        Ok(Self(SerdeSecret(sk)))
    }

    /// Convert a `PrivateKey` to bytes
    pub fn as_bytes(&self) -> [u8; SK_SIZE] {
        self.0.to_bytes()
    }

    /// Generates a random `PrivateKey`
    pub fn random() -> Self {
        Self(SerdeSecret(blsttc::SecretKey::random()))
    }

    /// Retrieves the associated `PublicKey` of this `PrivateKey`
    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.0.public_key())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature_verification() {
        let sk = PrivateKey::random();
        let pk = sk.public_key();
        let msg = "test message";
        let sig = Signature::sign(&sk, msg);
        assert!(sig.verify(&pk, msg));
    }

    #[test]
    fn test_signature_serde() {
        let sk = PrivateKey::random();
        let pk = sk.public_key();
        let msg = "test message";
        let sig = Signature::sign(&sk, msg);

        let sig_ser = bincode::serialize(&sig);
        assert!(sig_ser.is_ok());

        let sig_de = bincode::deserialize::<Signature>(&sig_ser.unwrap());
        assert!(sig_de.is_ok());

        let ss = sig_de.unwrap();
        assert_eq!(sig, ss);
        assert!(ss.verify(&pk, msg));
    }

    #[test]
    fn test_public_key() {
        let sk = PrivateKey::random();
        let pk = sk.public_key();
        let pk_ser = bincode::serialize(&pk);
        assert!(pk_ser.is_ok());

        let pk_de = bincode::deserialize::<PublicKey>(&pk_ser.unwrap());
        assert!(pk_de.is_ok());
        assert_eq!(pk, pk_de.unwrap());
    }

    #[test]
    fn test_private_key() {
        let sk = PrivateKey::random();
        let sk_ser = bincode::serialize(&sk);
        assert!(sk_ser.is_ok());

        let sk_de = bincode::deserialize::<PrivateKey>(&sk_ser.unwrap());
        assert!(sk_de.is_ok());
        assert_eq!(sk, sk_de.unwrap());
    }
}
