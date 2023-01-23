use crate::crypto::hash::Hash;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Representation of a routing table
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RoutingTable {
    entries: HashMap<Hash, (Hash, usize)>,
    version: usize,
}

/// Representation of a shared routing table
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SharedRoutingTable {
    entries: HashMap<Hash, usize>,
}

impl SharedRoutingTable {
    /// Retrieves the shared routing table.
    pub fn shared_entries(&self) -> &HashMap<Hash, usize> {
        &self.entries
    }

    /// Get shared routing information for a node.
    pub fn shared_routing_info(&self, node_id: &Hash) -> Option<usize> {
        self.entries.get(node_id).copied()
    }
}

impl RoutingTable {
    /// Creates a new `RoutingTable`.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            version: 0,
        }
    }

    /// Retrieve shared routing table.
    pub fn get_shared(&self) -> SharedRoutingTable {
        let entries = self
            .entries
            .iter()
            .map(|(node_id, (_intermediate, hops))| (*node_id, *hops))
            .collect::<HashMap<Hash, usize>>();
        SharedRoutingTable { entries }
    }

    /// Retrieve routing information for a node.
    pub fn get_routing_info(&self, node_id: &Hash) -> Option<(Hash, usize)> {
        self.entries.get(node_id).copied()
    }

    /// Get all routing information for all nodes.
    pub fn entries(&self) -> &HashMap<Hash, (Hash, usize)> {
        &self.entries
    }

    /// Get a mutable reference to routing information for all nodes.
    pub fn entries_mut(&mut self) -> &mut HashMap<Hash, (Hash, usize)> {
        &mut self.entries
    }

    /// Checks the entries to see if it contains a specified node.
    pub fn has_node(&self, node_id: &Hash) -> bool {
        self.entries.contains_key(node_id)
    }

    /// Insert a new routing info for a node,
    /// while generating a random one.
    pub fn add_new_node(&mut self, node_id: &Hash) {
        let _ = self.entries.insert(*node_id, (Hash::random(), usize::MAX));
    }

    /// Insert a new routing info for a node as a direct connection.
    pub fn add_direct_connection(&mut self, node_id: &Hash) {
        let _ = self.entries.insert(*node_id, (*node_id, usize::MAX));
    }

    /// Bump version number of the routing table.
    pub fn increment_version(&mut self) {
        self.version += 1;
    }

    /// Retrieve version number of routing table
    pub fn version(&self) -> usize {
        self.version
    }
}
