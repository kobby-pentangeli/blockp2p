use bytebuffer::ByteBuffer;
use rand::{thread_rng, Rng};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

const DISPLAY_HASH_LEN: usize = 4;
const RANDOM_HASH_BUF: usize = 4096;

/// Representation of a hash
#[derive(Clone, Hash, Copy, Eq, PartialEq)]
pub struct Hash(pub blake3::Hash);

impl Hash {
    /// Creates a `Hash` from bytes
    pub fn from_bytes(data: &[u8]) -> Self {
        Self(blake3::hash(data))
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

struct HashVisitor;

impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(self.0.as_bytes())
    }
}

impl<'de> Visitor<'de> for HashVisitor {
    type Value = Hash;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a hash byte array")
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Hash::from_bytes(v))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Hash::from_bytes(v.as_bytes()))
    }
}

impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(HashVisitor)
    }
}
