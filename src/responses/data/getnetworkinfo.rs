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
    pub version: usize,
    // subversion: String,
    // protocol: usize,
    // localservices: String,
    // localservicesnames: Vec<String>,
    // localrelay: bool,
    // timeoffset: usize,
    // connections: usize,
    // connections_in: usize,
    // connections_out: usize,
    // networkactive: bool,
    // networks: Vec<Network>,
    // relayfee: usize,
    // incrementalfee: usize,
    // localaddresses: Vec<LocalAddress>,
    // warnings: String,
}
