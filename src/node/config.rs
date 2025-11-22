use std::net::SocketAddr;



pub enum NodeType {
    Entry,
    Middle,
    Exit,
    Relay,
}
// Define the main configuration structure
struct NodeConfig {

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
}


