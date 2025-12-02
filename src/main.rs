mod node;
use node::{NodeConfig, NodeType, LightningNode};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{

    println!("LightningNet is starting...");

    // FIX: Use fixed port 8080 instead of :0
    let config1 = NodeConfig::new(
        "test-node".to_string(), 
        "127.0.0.1:8080".parse()?,  // ← Changed from :0 to :8080
        NodeType::Entry);

    println!("The node config created {:?}", config1);    
    println!("Node type: {:?}", config1.node_type);
    println!("Port: {}", config1.port());
    println!("Is public: {}", config1.is_public());

    let mut node1 = LightningNode::new(config1);
    
    // Start node1
    match node1.start().await {
        Ok(()) => println!("✅ Node started successfully"),
        Err(e) => println!("❌ Failed to start node: {}", e),
    }
    
    // Show initial status
    let status = node1.get_status();
    println!("📊 Node 1 status: {:?}", status);
    
    // Create second node configuration
    let config2 = NodeConfig::new(
        "node-2".to_string(), 
        "127.0.0.1:8081".parse()?,
        NodeType::Middle,
    );
    
    let mut node2 = LightningNode::new(config2);
    
    // Start second node and connect to first
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        
        match node2.start().await {
            Ok(()) => println!("✅ Node 2 started successfully"),
            Err(e) => println!("❌ Failed to start node 2: {}", e),
        }
        
        // Connect to node1 on the fixed port 8080
        match node2.connect_to_node("127.0.0.1:8080".parse().unwrap()).await {
            Ok(()) => println!("✅ Node 2 connected to Node 1"),
            Err(e) => println!("❌ Node 2 failed to connect: {}", e),
        }
    });
    
    // Let nodes run for a bit
    println!("🕐 Letting nodes run for 10 seconds...");
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    
    // Stop node1
    match node1.stop().await {
        Ok(()) => println!("✅ Node 1 stopped successfully"),
        Err(e) => println!("❌ Failed to stop node 1: {}", e),
    }
    
    println!("🎉 LightningNet demo completed!");
    Ok(())
}