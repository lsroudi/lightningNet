use std::net::SocketAddr;



// Define the main configuration structure
struct NodeConfig {

    pub node_id: String,
    pub listen_address: SocketAddr ,
    pub node_type: NodeType,

}

pub enum NodeType {
    Entry,
    Middle,
    Exit,
    Relay,
}