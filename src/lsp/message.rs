use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    jsonrpc: String,
    id: usize,
    method: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    jsonrpc: String,
    id: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Notification {
    jsonrpc: String,
    method: String,
}
