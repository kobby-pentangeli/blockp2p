use crate::{BlockP2pError, BlockP2pResult};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

/// BLS public key
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct PublicKey(bls_signatures::PublicKey);

impl PublicKey {
    /// Generates a `PublicKey` from bytes
    pub fn from_bytes(data: &[u8]) -> BlockP2pResult<Self> {
        use bls_signatures::Serialize;
        let pk = bls_signatures::PublicKey::from_bytes(data)
            .map_err(|e| BlockP2pError::BlsPublicKeyError(e.to_string()))?;
        Ok(Self(pk))
    }

    /// Convert a `PublicKey` to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        use bls_signatures::Serialize;
        self.0.as_bytes()
    }
}

/// BLS private key
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct PrivateKey(bls_signatures::PrivateKey);

impl PrivateKey {
    /// Generates a `PrivateKey` from bytes
    pub fn from_bytes(data: &[u8]) -> BlockP2pResult<Self> {
        use bls_signatures::Serialize;
        let pk = bls_signatures::PrivateKey::from_bytes(data)
            .map_err(|e| BlockP2pError::BlsPrivateKeyError(e.to_string()))?;
        Ok(Self(pk))
    }

    /// Convert a `PrivateKey` to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        use bls_signatures::Serialize;
        self.0.as_bytes()
    }

    /// Generates a random `PrivateKey`
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self(bls_signatures::PrivateKey::generate(&mut rng))
    }

    /// Retrieves the associated `PublicKey` of this `PrivateKey`
    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.0.public_key())
    }
}

struct PublicKeyVisitor;
struct PrivateKeyVisitor;

//************************** impl Serialize */
impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.to_bytes())
    }
}

impl Serialize for PrivateKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.to_bytes())
    }
}

//************************** impl Deserialize */
impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(PublicKeyVisitor)
    }
}

impl<'de> Deserialize<'de> for PrivateKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(PrivateKeyVisitor)
    }
}

//************************** impl Visitor */
impl<'de> Visitor<'de> for PublicKeyVisitor {
    type Value = PublicKey;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a signature byte array")
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        PublicKey::from_bytes(v).map_err(|e| E::custom(e.to_string()))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        PublicKey::from_bytes(v.as_bytes()).map_err(|e| E::custom(e.to_string()))
    }
}

impl<'de> Visitor<'de> for PrivateKeyVisitor {
    type Value = PrivateKey;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a signature byte array")
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        PrivateKey::from_bytes(v).map_err(|e| E::custom(e.to_string()))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        PrivateKey::from_bytes(v.as_bytes()).map_err(|e| E::custom(e.to_string()))
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
