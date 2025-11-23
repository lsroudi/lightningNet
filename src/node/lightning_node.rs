
use super::config::{NodeConfig};
use std::net::SocketAddr;
use std::collections::HashMap;


pub struct LightningNode {
    config: NodeConfig,
    active_connections: HashMap<SocketAddr,()>,
    circuits: HashMap<u32,()>,
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
    pub fn new(config: NodeConnfig)->Self{
        Self{
            config,
            active_connections: HashMap::new(),
            circuits: HashMap::new(),
            is_running: false,
        }
    }

    pub fn start(&mut self)->Result<(), String>{

        if self.is_running{
            return Err("Node is already running".to_string);
        }

        self.is_running = true;

        println!("Node {} started successfully", self.config.node_id);
        Ok(())

    }

    pub fn stop(&mut self)->Result<((),String)>{
        
        if !self.is_running{
            return Err("Node is already stopped".to_string);
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