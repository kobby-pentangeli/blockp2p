use super::key::{PrivateKey, PublicKey};
use crate::{BlockP2pError, BlockP2pResult};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

/// BLS signature
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Signature(pub bls_signatures::Signature);

impl Signature {
    /// Creates a new `Signature` from a BLS signature
    pub fn new(sig: bls_signatures::Signature) -> Self {
        Self(sig)
    }

    /// Sign a message
    pub fn sign<T: AsRef<[u8]>>(private_key: &PrivateKey, data: T) -> Self {
        Self::new(private_key.0.sign(data))
    }

    /// Creates a `Signature` from bytes
    pub fn from_bytes(data: &[u8]) -> BlockP2pResult<Self> {
        use bls_signatures::Serialize;
        let sig = bls_signatures::Signature::from_bytes(data)
            .map_err(|e| BlockP2pError::BlsSignatureError(e.to_string()))?;
        Ok(Self(sig))
    }

    /// Convert a `Signature` to bytes
    pub fn as_bytes(&self) -> Vec<u8> {
        use bls_signatures::Serialize;
        self.0.as_bytes()
    }

    /// Verify a message
    pub fn verify<T: AsRef<[u8]>>(&self, pub_key: &PublicKey, data: T) -> bool {
        pub_key.0.verify(self.0, data)
    }

    /// Aggregate signatures
    pub fn aggregate(signatures: &[Self]) -> BlockP2pResult<Self> {
        use bls_signatures::Signature as BlsSignature;
        let mut sigs: Vec<BlsSignature> = vec![];
        signatures.iter().for_each(|s| sigs.push(s.0));

        let agg = bls_signatures::aggregate(&sigs)
            .map_err(|e| BlockP2pError::BlsSignatureError(e.to_string()))?;
        Ok(Self::new(agg))
    }
}

//************************** impl Serialize */
impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.as_bytes())
    }
}

//************************** impl Deserialize */
impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(SignatureVisitor)
    }
}

//************************** impl Visitor */
struct SignatureVisitor;

impl<'de> Visitor<'de> for SignatureVisitor {
    type Value = Signature;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a signature byte array")
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Signature::from_bytes(v).map_err(|e| E::custom(e.to_string()))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Signature::from_bytes(v.as_bytes()).map_err(|e| E::custom(e.to_string()))
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
