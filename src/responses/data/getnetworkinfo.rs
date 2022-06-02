//! a bitcoind networkinfo field

use mentat::serde::Deserialize;

// #[derive(Clone, Debug, Deserialize)]
// #[serde(crate = "mentat::serde")]
// pub struct Network {
//     name: String,
//     limited: bool,
//     reachable: bool,
//     proxy: String,
//     proxy_randomize_credentials: bool,
// }

// #[derive(Clone, Debug, Deserialize)]
// #[serde(crate = "mentat::serde")]
// pub struct LocalAddress {
//     address: String,
//     port: usize,
//     score: usize,
// }

/// a bitcoind networkinfo field
#[allow(non_snake_case, clippy::missing_docs_in_private_items)]
#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct GetNetworkInfo {
    // pub version: usize,
    pub subversion: String,
    // protocol: usize,
    // localservices: String,
    // localservicesnames: Vec<String>,
    // localrelay: bool,
    // timeoffset: usize,
    pub connections: u64,
    pub connections_in: u64,
    pub connections_out: u64,
    // networkactive: bool,
    // networks: Vec<Network>,
    // relayfee: usize,
    // incrementalfee: usize,
    // localaddresses: Vec<LocalAddress>,
    // warnings: String,
}

impl GetNetworkInfo {
    /// parse the node version from the network info
    pub fn version(&self) -> &str {
        self.subversion
            .strip_prefix("/Satoshi:")
            .unwrap()
            .strip_suffix('/')
            .unwrap()
    }
}
