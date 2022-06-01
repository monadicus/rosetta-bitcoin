//! optional mentat endpoints for bitcoind
use mentat::{api::OptionalApi, axum::async_trait};

/// optional mentat endpoints for bitcoind
#[derive(Clone, Default)]
pub struct BitcoinOptionalApi;

#[async_trait]
impl OptionalApi for BitcoinOptionalApi {}
