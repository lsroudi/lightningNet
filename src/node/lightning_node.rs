
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
    pub listen_address: SocketAddr,
}

impl LightningNode {
    pub fn new(config: NodeConfig)->Self{
        Self{
            config,
            listener:None,
            active_connections: HashMap::new(),
            circuits: HashMap::new(),
            is_running: false,
        }
    }

    pub async fn start(&mut self) -> Result<(), String> {
        if self.is_running {
            return Err("Node is already running".to_string());
        }
        
        println!("Starting LightningNode: {}", self.config.node_id);
        
        // Bind to the configured address
        let listener = TcpListener::bind(&self.config.listen_address)
            .await
            .map_err(|e| format!("Failed to bind to {}: {}", self.config.listen_address, e))?;
        
        let actual_address = listener.local_addr()
            .map_err(|e| format!("Failed to get local address: {}", e))?;
        
        println!("Node listening on: {}", actual_address);
        self.listener = Some(listener);
        self.is_running = true;

        // Start accepting connections in background task
        let listener_clone = self.listener.clone().unwrap();
        let node_id = self.config.node_id.clone();
        
        tokio::spawn(async move {
            if let Err(e) = Self::accept_connections_background(listener_clone, node_id).await {
                println!("Accept connections error: {}", e);
            }
        });
        
        Ok(())
    }

    /// Background task for accepting connections
    async fn accept_connections_background(listener: TcpListener, node_id: String) -> Result<(), String> {
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    println!("[{}] New connection from: {}", node_id, addr);
                    
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(stream, addr).await {
                            println!("[{}] Connection handler error: {}", node_id, e);
                        }
                    });
                }
                Err(e) => {
                    println!("[{}] Accept connection error: {}", node_id, e);
                }
            }
        }
    }

    pub async fn stop(&mut self)->Result<(),String>{
        
        if !self.is_running{
            return Err("Node is already stopped".to_string());
        }

        self.is_running = false;
        self.active_connections.clear();
        self.circuits.clear();

        println!("Node {} stopped successfully", self.config.node_id);

        Ok(())
    }

    async fn accept_connections(&mut self)->Result<(),String>{

        let listener = self.listener
            .as_ref()
            .ok_or("Listener not initialized".to_string())?;

        while self.is_running {

            match listener.accept().await {

                Ok((stream,addr)) => {
                    println!("New connection from: {}", addr);
                    
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(stream, addr).await {
                            println!("Connection handler error: {}", e);
                        }
                    });
                }
                
                Err(e) => {
                    println!("Accept connection error: {}", e);
                }
            }           
        }

        Ok(())
    }

    async fn handle_connection(mut stream: TcpStream, addr: SocketAddr) -> Result<(), String>{

        println!("Handling connection from: {}", addr);
        
        // For now, just send a welcome message
        let welcome_msg = format!("Welcome to LightningNet! Connected from: {}\n", addr);
        stream.writable().await
            .map_err(|e| format!("Stream not writable: {}", e))?;
            
        stream.try_write(welcome_msg.as_bytes())
            .map_err(|e| format!("Failed to write welcome message: {}", e))?;

        // Keep the connection open for a bit to demonstrate
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        println!("Connection from {} closed", addr);

        Ok(())
    }

    pub async fn connect_to_node(&mut self, address: SocketAddr) -> Result<(), String> {
        println!("Connecting to node at: {}", address);
        
        let stream = TcpStream::connect(address)
            .await
            .map_err(|e| format!("Failed to connect to {}: {}", address, e))?;
        
        println!("Connected to node at: {}", address);
        
        // Store the connection (simplified for now)
        self.active_connections.insert(address, mpsc::channel(32).0);
        
        // Handle the connection
        tokio::spawn(async move {
            if let Err(e) = Self::handle_connection(stream, address).await {
                println!("Outgoing connection error: {}", e);
            }
        });

        Ok(())
    }

    pub fn get_status(&self) -> NodeStatus {
        NodeStatus {
            node_id: self.config.node_id.clone(),
            is_running: self.is_running,
            connections: self.active_connections.len(),
            circuits: self.circuits.len(),
            node_type: self.config.node_type.clone(),
            listen_address: self.config.listen_address,
        }
    }
}