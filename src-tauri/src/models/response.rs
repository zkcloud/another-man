use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub time_ms: u64,
    pub size_bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseError {
    pub message: String,
    pub kind: ErrorKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorKind {
    Network,
    Timeout,
    InvalidUrl,
    Other,
}