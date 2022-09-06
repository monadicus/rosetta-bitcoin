//! rosetta events api implementation for bitcoin using mentat

use super::*;

/// rosetta data routes for bitcoin
#[derive(Clone, Default)]
pub struct BitcoinEventsApi;

#[async_trait]
impl EventsApiRouter for BitcoinEventsApi {}

#[async_trait]
impl EventsApi for BitcoinEventsApi {
    type NodeCaller = BitcoinCaller;
}
