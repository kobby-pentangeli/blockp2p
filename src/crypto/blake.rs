use serde::{Deserialize, Serialize};

const LONG_HASH_LEN: usize = 32;
const SHORT_HASH_LEN: usize = 20;

/// Representation of a Blake hash
#[derive(Debug, Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Blake {}

impl Blake {
    /// Creates a (long) Blake hash from bytes
    pub fn long(src: &[u8]) -> [u8; LONG_HASH_LEN] {
        let blake = Self::create_hash(src, LONG_HASH_LEN);
        let mut hash: [u8; LONG_HASH_LEN] = [0; LONG_HASH_LEN];
        hash.copy_from_slice(&blake[0..LONG_HASH_LEN]);
        hash
    }

    /// Creates a (short) Blake hash from bytes
    pub fn short(src: &[u8]) -> [u8; SHORT_HASH_LEN] {
        let blake = Self::create_hash(src, SHORT_HASH_LEN);
        let mut hash: [u8; SHORT_HASH_LEN] = [0; SHORT_HASH_LEN];
        hash.copy_from_slice(&blake[0..SHORT_HASH_LEN]);
        hash
    }

    /// Create a Blake hash from bytes and a length param
    fn create_hash(src: &[u8], len: usize) -> Vec<u8> {
        blake2b_simd::Params::new()
            .hash_length(len)
            .to_state()
            .update(src)
            .finalize()
            .as_ref()
            .to_vec()
    }
}
