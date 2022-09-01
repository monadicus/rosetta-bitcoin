//! rpc requests

use mentat_server::{
    serde::Serialize,
    serde_json::{json, Value},
};

/// helper function to trim `0x` from hashes
pub fn trim_hash(hash: &str) -> &str {
    if let Some(h) = hash.strip_prefix("0x") {
        h
    } else {
        hash
    }
}

/// the rpc request structure for bitcoin
#[derive(Debug, Serialize)]
#[serde(crate = "mentat_server::serde")]
pub struct BitcoinJrpc {
    /// rpc info
    jsonrpc: String,
    /// id
    id: String,
    /// endpoint
    method: String,
    /// json arguments
    params: Vec<Value>,
}

impl BitcoinJrpc {
    /// create a new jrpc request for bitcoin
    pub fn new<P: Serialize>(method: &str, params: &[P]) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: method.to_string(),
            params: params.iter().map(|p| json!(p)).collect(),
        }
    }
}

/// request object for `account/balance` endpoint
#[derive(Debug, Serialize)]
#[serde(crate = "mentat_server::serde")]
pub struct ScanObjectsDescriptor {
    /// account id
    pub desc: String,
    /// block end range
    pub range: i64,
}
