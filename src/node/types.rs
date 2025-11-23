

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum NodeType {
    Entry,
    Middle,
    Exit,
    Relay,
}