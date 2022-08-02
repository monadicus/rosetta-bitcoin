//! a bitcoind getblockchaininfo response

use mentat_server::serde::Deserialize;
use mentat_types::{NetworkIdentifier, NetworkListResponse};

/// a bitcoind getblockchaininfo response
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Debug, Deserialize)]
#[serde(crate = "mentat_server::serde")]
pub struct GetBlockchainInfoResponse {
    pub blocks: usize,
    pub chain: String,
    pub headers: usize,
}

impl From<GetBlockchainInfoResponse> for NetworkListResponse {
    fn from(info: GetBlockchainInfoResponse) -> Self {
        Self {
            network_identifiers: vec![NetworkIdentifier {
                blockchain: String::from("bitcoin"),
                network: match info.chain.as_ref() {
                    "main" => String::from("mainnet"),
                    "test" => String::from("testnet"),
                    _ => info.chain,
                },
                // rosetta didnt include this in their btc impl
                sub_network_identifier: None,
            }],
        }
    }
}
