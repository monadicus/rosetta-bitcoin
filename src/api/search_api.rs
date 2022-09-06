//! rosetta search api implementation for bitcoin using mentat

use super::*;

/// rosetta data routes for bitcoin
#[derive(Clone, Default)]
pub struct BitcoinSearchApi;

#[async_trait]
impl SearchApiRouter for BitcoinSearchApi {}

#[async_trait]
impl SearchApi for BitcoinSearchApi {
    type NodeCaller = BitcoinCaller;
}
