mod node;

use node::config::{NodeConfig};
use node::types::{NodeType};
use std::net::SocketAddr;

fn main() {
    println!("LightningNet is starting...");

    // Test our config
    let config = NodeConfig::new(
        "test-node".to_string(), 
        "127.0.0.1:8080".parse().unwrap(), 
        NodeType::Entry);

    println!("The node config created {:?}", config);    
    println!("Node type: {:?}", config.node_type);
    println!("Port: {}", config.port());
    println!("Is public: {}", config.is_public());
}
