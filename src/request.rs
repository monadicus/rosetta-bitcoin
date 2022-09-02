//! rpc requests

use std::fmt::Debug;

use mentat_server::{
    conf::NodeConf,
    macro_exports::Configuration,
    reqwest,
    serde::{de::DeserializeOwned, Serialize},
    serde_json::{self, json, Value},
};
use mentat_types::{MapErrMentat, Result};

use crate::{node::BitcoinConfig, responses::BitcoinResponse};

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

/// The `RpcCaller` struct is a wrapper to hold a rpc caller instance
/// that holds a request client and the url for the RPC.
#[derive(Clone, Debug)]
pub struct BitcoinCaller {
    /// The request client.
    pub client: reqwest::Client,
    /// The RPC url.
    pub node_rpc_url: reqwest::Url,
}

impl From<Configuration<BitcoinConfig>> for BitcoinCaller {
    fn from(conf: Configuration<BitcoinConfig>) -> Self {
        Self {
            client: reqwest::Client::new(),
            node_rpc_url: NodeConf::build_url(&conf),
        }
    }
}

impl BitcoinCaller {
    /// Makes the RPC call returning the expected output given the input type.
    pub async fn rpc_call<O: DeserializeOwned + Debug>(&self, req: BitcoinJrpc) -> Result<O> {
        let resp = self
            .client
            .post(self.node_rpc_url.clone())
            .json(&req)
            .send()
            .await?;

        let resp_text = resp.text().await?;
        let response_type = serde_json::from_str::<BitcoinResponse<O>>(&resp_text)
            .merr(|e| format!("failed to serialize response: `{e}`\ntext: `{resp_text}`"))?;
        response_type.into_result()
    }
}

/// helper function to trim `0x` from hashes
pub fn trim_hash(hash: &str) -> &str {
    if let Some(h) = hash.strip_prefix("0x") {
        h
    } else {
        hash
    }
}
