//! an example implementation of rosetta for bitcoin using mentat

#![deny(clippy::all, clippy::missing_docs_in_private_items)]
#![warn(clippy::todo)]

mod api;
use api::*;
mod node;
mod request;
mod responses;

use mentat_server::{mentat, server::ServerType};

/// The mentat rosetta-bitcoin.
#[mentat]
struct MentatBitcoin;

impl ServerType for MentatBitcoin {
    type AccountApi = BitcoinAccountApi;
    type BlockApi = BitcoinBlockApi;
    type CallApi = BitcoinCallApi;
    type ConstructionApi = BitcoinConstructionApi;
    type EventsApi = BitcoinEventsApi;
    type MempoolsApi = BitcoinMempoolApi;
    type NetworkApi = BitcoinNetworkApi;
    type OptionalApi = BitcoinOptionalApi;
    type SearchApi = BitcoinSearchApi;
    type CustomConfig = node::NodeConfig;
}
