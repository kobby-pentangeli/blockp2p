use super::blake::Blake;
use crate::{BlockP2pError, BlockP2pResult};
use bytebuffer::ByteBuffer;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

const DISPLAY_HASH_LEN: usize = 4;
const RANDOM_HASH_BUF: usize = 4096;

/// Representation of a hash
#[derive(Clone, Default, Hash, Copy, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Hash(pub [u8; 32]);

impl Hash {
    /// Creates a new `Hash` from bytes
    pub fn new(data: &[u8]) -> Self {
        Self(Blake::long(data))
    }

    /// Create a `Hash` for any serializable data
    pub fn serialize<S: Serialize>(data: &S) -> BlockP2pResult<Self> {
        let ser = bincode::serialize(data)
            .map_err(|e| BlockP2pError::SerializeHashError(e.to_string()))?;
        Ok(Self(Blake::long(&ser)))
    }

    /// Generates a random `Hash`
    pub fn random() -> Self {
        let mut bytes: [u8; RANDOM_HASH_BUF] = [0; RANDOM_HASH_BUF];
        thread_rng().fill(&mut bytes);
        Self(Blake::long(bytes.as_ref()))
    }

    /// Encode a `Hash` as a `Hex` string
    pub fn to_hex_string(&self) -> String {
        hex::encode(self.0)
    }

    /// Creates a `Hash` from byte arrays
    pub fn from_byte_arrays(ba: &[&[u8]]) -> Self {
        let mut buf = ByteBuffer::new();
        for b in ba {
            buf.write_bytes(b);
        }
        Self(Blake::long(buf.as_bytes()))
    }

    /// Convert a `Hash` to a `Vec<u8>`
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in 1..DISPLAY_HASH_LEN {
            s.push_str(&format!("{:02X}", self.0[i - 1]));
        }
        s.push_str("...");
        for i in (1..DISPLAY_HASH_LEN).rev() {
            s.push_str(&format!("{:02X}", self.0[32 - i]));
        }
        write!(f, "{}", s)
    }
}

impl std::fmt::Debug for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in 1..DISPLAY_HASH_LEN {
            s.push_str(&format!("{:02X}", self.0[i - 1]));
        }
        s.push_str("...");
        for i in (1..DISPLAY_HASH_LEN).rev() {
            s.push_str(&format!("{:02X}", self.0[32 - i]));
        }
        write!(f, "{}", s)
    }
}

/// Representation of a short hash
#[derive(
    Debug, Default, Clone, Hash, Copy, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub struct ShortHash(pub [u8; 20]);

impl ShortHash {
    /// Creates a new `ShortHash` from bytes
    pub fn new(data: &[u8]) -> Self {
        Self(Blake::short(data))
    }

    /// Create a `ShortHash` for any serializable data
    pub fn serialize<S: Serialize>(data: &S) -> BlockP2pResult<Self> {
        let ser = bincode::serialize(data)
            .map_err(|e| BlockP2pError::SerializeHashError(e.to_string()))?;
        Ok(Self(Blake::short(&ser)))
    }

    /// Generates a random `ShortHash`
    pub fn random() -> Self {
        let mut bytes: [u8; RANDOM_HASH_BUF] = [0; RANDOM_HASH_BUF];
        thread_rng().fill(&mut bytes);
        Self(Blake::short(bytes.as_ref()))
    }

    /// Encode a `ShortHash` as a `Hex` string
    pub fn to_hex_string(&self) -> String {
        hex::encode(self.0)
    }

    /// Creates a `ShortHash` from byte arrays
    pub fn from_byte_arrays(ba: &[&[u8]]) -> Self {
        let mut buf = ByteBuffer::new();
        for b in ba {
            buf.write_bytes(b);
        }
        Self(Blake::short(buf.as_bytes()))
    }

    /// Convert a `ShortHash` to a `Vec<u8>`
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl AsRef<[u8]> for ShortHash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

// /// Hash type representation.
// /// Used for common Hash relations.
// pub trait HashType:
//     Eq
//     + Ord
//     + Clone
//     + std::fmt::Debug
//     + Send
//     + Serialize
//     + DeserializeOwned
//     + Sync
//     + std::hash::Hash
//     + std::fmt::Display
//     + Default
// {
// }

// impl<N> HashType for N where
//     N: Eq
//         + Ord
//         + Clone
//         + Send
//         + std::fmt::Debug
//         + std::fmt::Display
//         + std::hash::Hash
//         + Serialize
//         + DeserializeOwned
//         + Sync
//         + Default
// {
// }
