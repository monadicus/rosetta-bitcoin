//! rosetta indexer api implementation for bitcoind using mentat

use mentat::{
    api::{CallerIndexerApi, IndexerApi},
    axum::async_trait,
};

/// rosetta indexer routes for bitcoind
#[derive(Clone, Default)]
pub struct BitcoinIndexerApi;

#[async_trait]
impl CallerIndexerApi for BitcoinIndexerApi {}

#[async_trait]
impl IndexerApi for BitcoinIndexerApi {}
