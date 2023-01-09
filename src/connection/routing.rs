use crate::crypto::hash::Hash;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Representation of a routing table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingTable {
    entries: HashMap<Hash, (Hash, usize)>,
    version: usize,
}
