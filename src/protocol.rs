use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientRequest {
    pub protocol_version: u16,
    pub request_id: String,
    pub prompt: String,
    pub cwd: String,
    pub shell: Option<String>,
    pub env: Vec<(String, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerResponse {
    pub protocol_version: u16,
    pub request_id: String,
    pub command: Option<String>,
    pub explanation: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PingRequest {
    pub protocol_version: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PingResponse {
    pub protocol_version: u16,
    pub ok: bool,
}
