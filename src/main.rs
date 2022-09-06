//! an example implementation of rosetta for bitcoin using mentat

#![deny(clippy::all, clippy::missing_docs_in_private_items)]
#![warn(clippy::todo)]

mod api;
use api::*;
mod node;
use node::BitcoinConfig;
mod request;
use request::BitcoinCaller;
mod responses;

use mentat_asserter::Asserter;
use mentat_server::{
    conf::{AsserterTable, Configuration, NodeConf},
    mentat,
    server::ServerType,
};

/// The mentat rosetta-bitcoin.
#[mentat]
struct MentatBitcoin;

impl ServerType for MentatBitcoin {
    type AccountApi = BitcoinAccountApi;
    type BlockApi = BitcoinBlockApi;
    type CallApi = BitcoinCallApi;
    type ConstructionApi = BitcoinConstructionApi;
    type CustomConfig = BitcoinConfig;
    type EventsApi = BitcoinEventsApi;
    type MempoolsApi = BitcoinMempoolApi;
    type NetworkApi = BitcoinNetworkApi;
    type NodeCaller = BitcoinCaller;
    type OptionalApi = BitcoinOptionalApi;
    type SearchApi = BitcoinSearchApi;

    fn init_asserters(config: &Configuration<Self::CustomConfig>) -> AsserterTable {
        Asserter::new_server(
            vec!["INPUT".into(), "OUTPUT".into(), "COINBASE".into()],
            true,
            vec![
                (
                    Self::CustomConfig::BLOCKCHAIN,
                    config.network.to_string().as_str(),
                )
                    .into(),
            ],
            Vec::new(),
            false,
            None,
        )
        .unwrap()
        .into()
    }
}
