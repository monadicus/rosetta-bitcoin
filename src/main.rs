//! an example implementation of rosetta for bitcoin using mentat

#![deny(clippy::all, clippy::missing_docs_in_private_items)]
#![warn(clippy::todo)]

mod call_api;
mod construction_api;
mod data_api;
mod indexer_api;
mod node;
mod optional_api;
mod request;
mod responses;

use mentat::{mentat, server::ServerType};

/// a rosetta implementation for bitcoin using mentat
// #[mentat(DefaultCacheInner)] caching is broken
#[mentat]
struct MentatBitcoin;

impl ServerType for MentatBitcoin {
    type CallApi = call_api::BitcoinCallApi;
    type ConstructionApi = construction_api::BitcoinConstructionApi;
    type CustomConfig = node::NodeConfig;
    type DataApi = data_api::BitcoinDataApi;
    type IndexerApi = indexer_api::BitcoinIndexerApi;
    type OptionalApi = optional_api::BitcoinOptionalApi;
}
