use crate::{BlockP2pError, BlockP2pResult};
use bytebuffer::ByteBuffer;
use rand::{thread_rng, Rng};
use serde::Serialize;

const DISPLAY_HASH_LEN: usize = 4;
const RANDOM_HASH_BUF: usize = 4096;

/// Representation of a hash
#[derive(Clone, Hash, Copy, Eq, PartialEq)]
pub struct Hash(pub blake3::Hash);

impl Hash {
    /// Creates a new `Hash` from bytes
    pub fn new(data: &[u8]) -> Self {
        Self(blake3::hash(data))
    }

    /// Create a `Hash` for any serializable data
    pub fn serialize<S: Serialize>(data: &S) -> BlockP2pResult<Self> {
        let ser = bincode::serialize(data)
            .map_err(|e| BlockP2pError::SerializeHashError(e.to_string()))?;
        Ok(Self(blake3::hash(&ser)))
    }

    /// Generates a random `Hash`
    pub fn random() -> Self {
        let mut bytes: [u8; RANDOM_HASH_BUF] = [0; RANDOM_HASH_BUF];
        thread_rng().fill(&mut bytes);
        Self(blake3::hash(bytes.as_ref()))
    }

    /// Encode a `Hash` as a hex string
    pub fn to_hex_string(&self) -> String {
        blake3::Hash::to_hex(&self.0).to_string()
    }

    /// Creates a `Hash` from byte arrays
    pub fn from_byte_arrays(ba: &[&[u8]]) -> Self {
        let mut buf = ByteBuffer::new();
        for b in ba {
            buf.write_bytes(b);
        }
        Self(blake3::hash(buf.as_bytes()))
    }

    /// Convert a `Hash` to a `Vec<u8>`
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in 1..DISPLAY_HASH_LEN {
            s.push_str(&format!("{:02X}", self.as_ref()[i - 1]));
        }
        s.push_str("...");
        for i in (1..DISPLAY_HASH_LEN).rev() {
            s.push_str(&format!("{:02X}", self.as_ref()[32 - i]));
        }
        write!(f, "{}", s)
    }
}

impl std::fmt::Debug for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in 1..DISPLAY_HASH_LEN {
            s.push_str(&format!("{:02X}", self.as_ref()[i - 1]));
        }
        s.push_str("...");
        for i in (1..DISPLAY_HASH_LEN).rev() {
            s.push_str(&format!("{:02X}", self.as_ref()[32 - i]));
        }
        write!(f, "{}", s)
    }
}
