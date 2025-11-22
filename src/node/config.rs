use std::net::SocketAddr;


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum NodeType {
    Entry,
    Middle,
    Exit,
    Relay,
}
// Define the main configuration structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeConfig {

    pub node_id: String,
    pub listen_address: SocketAddr ,
    pub node_type: NodeType,
    pub max_connections: usize,
    pub public_address: Option<SocketAddr>,
    pub storage_path: Option<String>,

}

impl Default for NodeConfig{
    fn default() -> Self {
        Self{

            node_id: "unconfigured".to_string(),
            listen_address: "127.0.0.1:0".parse().unwrap() ,
            node_type: NodeType::Relay,
            max_connections: 100,
            public_address: None,
            storage_path: Some("./data".to_string()),      
        }        
    }
}

impl NodeConfig{

    pub fn new(
        node_id: String,
        listen_address: SocketAddr,
        node_type: NodeType)-> Self{
            Self{
                node_id,
                listen_address,
                node_type,
                ..Default::default()
            }
    }

    pub fn is_public(&self)->bool{
        self.public_address.is_some()
    }

    pub fn port(&self)->u16{
        self.listen_address.port()
    }

    pub fn generate_random_id()->String{
        use rand::Rng;
        let random_byte: [u8;8] = rand::thread_rng().gen();
        hex::encode(random_byte)
    }

    pub fn validate(&self)-> Result<(),String>{

        if self.node_id == "unconfigured"{
            return Err("Node ID must be configured".to_string());
        }
        if self.max_connections == 0 {
            return Err("Max connections must be greater than 0".to_string());
        }

        if let Some(path) = &self.storage_path {
            if path.is_empty(){
                return Err("Storage path cannot be empty".to_string());
            }
        }

        Ok(())
    }
}


