mod node;
use node::{NodeConfig, NodeType, LightningNode};
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

    let mut node = LightningNode::new(config);
    // Test node operations
    match node.start() {
        Ok(()) => println!("✅ Node started successfully"),
        Err(e) => println!("❌ Failed to start node: {}", e),
    }
    
    // Check status
    let status = node.get_status();
    println!("📊 Node status: {:?}", status);
    
    // Test stopping
    match node.stop() {
        Ok(()) => println!("✅ Node stopped successfully"),
        Err(e) => println!("❌ Failed to stop node: {}", e),
    }
}
