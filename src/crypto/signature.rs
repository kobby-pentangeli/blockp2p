use super::key::{PrivateKey, PublicKey};
use crate::{BlockP2pError, BlockP2pResult};
use blsttc::SIG_SIZE;
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
    pub fn from_bytes(data: [u8; SIG_SIZE]) -> BlockP2pResult<Self> {
        let sig = blsttc::Signature::from_bytes(data)
            .map_err(|e| BlockP2pError::BlsSignatureError(e.to_string()))?;
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
}
