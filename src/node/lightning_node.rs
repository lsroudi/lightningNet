
use super::config::{NodeConfig};
use super::types::{NodeType};
use std::net::SocketAddr;
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum NetworkMessage {
    Ping {nonce: u64},
    Pong {nonce: u64},
    PeerAdvertisment {
        node_id: String,
        address: SocketAddr,
        node_type: String,
    },
}
#[derive(Debug, Clone)]
pub struct Circuit{
    id: u32,
    route:Vec<SocketAddr>,
    established: bool,
    created_at: std::time::Instant,
}
pub struct LightningNode {
    config: NodeConfig,
    active_connections: HashMap<SocketAddr,mpsc::Sender<()>>,
    circuits: HashMap<u32,Circuit>,
    listener: Option<TcpListener>,
    is_running: bool,
}

#[derive(Debug, Clone)]
pub struct NodeStatus {
    pub node_id: String,
    pub is_running: bool,
    pub connections: usize,
    pub circuits: usize,
    pub node_type: NodeType,
}

impl LightningNode {
    pub fn new(config: NodeConfig)->Self{
        Self{
            config,
            active_connections: HashMap::new(),
            circuits: HashMap::new(),
            is_running: false,
        }
    }

    pub async fn start(&mut self)->Result<(), String>{

        if self.is_running{
            return Err("Node is already running".to_string());
        }

        println!("Starting LightningNode: {}", self.config.node_id);

        let listener = TcpListener::bind(&self.config.listen_address)
            .await
            .map_err(|e| format!("Failed to bind to {}: {}", self.config.listen_address, e))?;

            let actual_address = listener.local_addr()
            .map_err(|e| format!("Failed to get local address: {}", e))?;

        println!("Node listening on: {}", actual_address);

        self.listener = Some(listener);

        self.is_running = true;

        // Start accepting connections
        self.accept_connections().await?;

        Ok(())

    }

    pub fn stop(&mut self)->Result<(),String>{
        
        if !self.is_running{
            return Err("Node is already stopped".to_string());
        }

        self.is_running = false;
        self.active_connections.clear();
        self.circuits.clear();

        println!("Node {} stopped successfully", self.config.node_id);

        Ok(())
    }

    pub fn get_status(&self) -> NodeStatus {
        NodeStatus {
            node_id: self.config.node_id.clone(),
            is_running: self.is_running,
            connections: self.active_connections.len(),
            circuits: self.circuits.len(),
            node_type: self.config.node_type.clone(),
        }
    }
}