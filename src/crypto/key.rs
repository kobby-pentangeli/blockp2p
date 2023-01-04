use crate::{BlockP2pError, BlockP2pResult};
use blsttc::{serde_impl::SerdeSecret, PK_SIZE, SK_SIZE};
use serde::{Deserialize, Serialize};

/// A `blsttc` public key
#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub struct PublicKey(pub blsttc::PublicKey);

impl PublicKey {
    /// Generates a `PublicKey` from bytes
    pub fn from_bytes(data: [u8; PK_SIZE]) -> BlockP2pResult<Self> {
        let pk = blsttc::PublicKey::from_bytes(data)
            .map_err(|e| BlockP2pError::BlsPublicKeyError(e.to_string()))?;
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
    pub fn from_bytes(data: [u8; SK_SIZE]) -> BlockP2pResult<Self> {
        let sk = blsttc::SecretKey::from_bytes(data)
            .map_err(|e| BlockP2pError::BlsPrivateKeyError(e.to_string()))?;
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
