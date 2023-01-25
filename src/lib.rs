//! BlockP2P:
//! Fast blockchain broadcast with scalable peer-to-peer network topology.
#![forbid(
    arithmetic_overflow,
    mutable_transmutes,
    no_mangle_const_items,
    unknown_crate_types
)]
#![warn(
    bad_style,
    deprecated,
    improper_ctypes,
    missing_docs,
    non_shorthand_field_patterns,
    overflowing_literals,
    stable_features,
    unconditional_recursion,
    unknown_lints,
    unsafe_code,
    unused,
    unused_allocation,
    unused_attributes,
    unused_comparisons,
    unused_features,
    unused_parens,
    while_true,
    clippy::unicode_not_nfc,
    clippy::unwrap_used,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

/// Configuration of a node
pub mod config;
/// Routing and connection protocol
pub mod connection;
/// Cryptographic primitives
pub mod crypto;
/// Implements error types
pub mod error;
/// Node and network-related events
pub mod event;
/// Identity of a node
pub mod identity;
/// Messaging protocol
pub mod messaging;
/// Functionality of a node on the network
pub mod node;

pub use config::Config;
pub use connection::{
    routing::{RoutingTable, SharedRoutingTable},
    Connection,
};
pub use event::Event;
pub use identity::{keys::Keys, Identity};
pub use messaging::{message::Message, Messaging};

/// Result wrapper
pub type Result<T> = std::result::Result<T, error::Error>;
