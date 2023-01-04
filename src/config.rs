use qp2p::Config as QuicConfig;
use std::{
    collections::{hash_set::Iter, HashSet},
    net::SocketAddr,
};
use structopt::StructOpt;

/// Configuration of a p2p node
#[derive(StructOpt, Debug, Default, Clone)]
#[structopt(rename_all = "kebab-case")]
pub struct Config {
    /// Bootstrap nodes
    #[structopt(short, long, default_value = "[]", parse(try_from_str = serde_json::from_str))]
    bootstrap_nodes: HashSet<SocketAddr>,
    /// QUIC config
    #[structopt(flatten)]
    quic: QuicConfig,
    /// Deploy agent
    #[structopt(short, long)]
    deploy_agent: bool,
}

impl Config {
    /// Retrieve the QUIC configuration
    pub fn get_quic_config(&self) -> &QuicConfig {
        &self.quic
    }

    /// Set the QUIC  configuration
    pub fn set_quic_config(&mut self, config: QuicConfig) {
        self.quic = config;
    }

    /// Retrieve bootstrap nodes
    pub fn get_bootstrap_nodes(&self) -> Iter<SocketAddr> {
        self.bootstrap_nodes.iter()
    }

    /// Get a mutable reference to the bootstrap nodes
    pub fn bootstrap_nodes_mut(&mut self) -> &mut HashSet<SocketAddr> {
        &mut self.bootstrap_nodes
    }

    /// Set bootstrap nodes
    pub fn add_bootstrap_nodes<P>(&mut self, peers: P)
    where
        P: IntoIterator<Item = SocketAddr>,
    {
        self.bootstrap_nodes.extend(peers);
    }

    /// Allow an agent to be deployed
    pub fn deploy_agent(&mut self) {
        self.deploy_agent = true;
    }

    /// Checks if an agent should be deployed from this node
    pub fn should_deploy(&self) -> bool {
        self.deploy_agent
    }
}
