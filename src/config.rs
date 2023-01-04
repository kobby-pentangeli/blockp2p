use qp2p::Config as QuicConfig;
use std::net::SocketAddr;
use structopt::StructOpt;

/// Configuration of a p2p node
#[derive(StructOpt, Debug, Default, Clone)]
#[structopt(rename_all = "kebab-case")]
pub struct Config {
    /// Identity of a node.
    /// A random one is created if `None` is specified.
    #[structopt(short, long)]
    identity: Option<String>,
    /// Is this node the genesis node?
    #[structopt(short, long)]
    genesis: bool,
    /// QUIC config
    #[structopt(flatten)]
    quic: QuicConfig,
    /// Deploy agent
    #[structopt(short, long)]
    deploy_agent: bool,
    /// Bootstrap nodes
    #[structopt(short, long, default_value = "[]", parse(try_from_str = serde_json::from_str))]
    bootstrap_nodes: Vec<SocketAddr>,
}

impl Config {
    /// Checks if node is genesis node
    pub fn is_genesis_node(&self) -> bool {
        self.genesis
    }

    /// Retrieves the identity of a node
    pub fn identity(&self) -> &Option<String> {
        &self.identity
    }

    /// Retrieve the QUIC configuration
    pub fn quic_config(&self) -> &QuicConfig {
        &self.quic
    }

    /// Set the QUIC  configuration
    pub fn set_quic_config(&mut self, config: QuicConfig) {
        self.quic = config;
    }

    /// Retrieves bootstrap nodes
    pub fn bootstrap_nodes(&self) -> std::slice::Iter<SocketAddr> {
        self.bootstrap_nodes.iter()
    }

    /// Get a mutable reference to the bootstrap nodes
    pub fn bootstrap_nodes_mut(&mut self) -> &mut Vec<SocketAddr> {
        &mut self.bootstrap_nodes
    }

    /// Set bootstrap nodes
    pub fn add_bootstrap_nodes<P>(&mut self, peers: P)
    where
        P: IntoIterator<Item = SocketAddr>,
    {
        self.bootstrap_nodes.extend(peers);
    }

    /// Checks if an agent should be deployed from this node
    pub fn should_deploy(&self) -> bool {
        self.deploy_agent
    }
}
