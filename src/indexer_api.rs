//! rosetta indexer api implementation for bitcoind using mentat

use mentat::{
    api::{Caller, CallerIndexerApi, IndexerApi, MentatResponse},
    axum::async_trait,
    requests::*,
    responses::*,
    server::RpcCaller,
};

/// rosetta indexer routes for bitcoind
#[derive(Clone, Default)]
pub struct BitcoinIndexerApi;

#[async_trait]
impl CallerIndexerApi for BitcoinIndexerApi {}

#[async_trait]
impl IndexerApi for BitcoinIndexerApi {
    async fn events_blocks(
        &self,
        _caller: Caller,
        _data: EventsBlocksRequest,
        _rpc_caller: RpcCaller,
    ) -> MentatResponse<EventsBlocksResponse> {
        todo!()
    }

    async fn search_transactions(
        &self,
        _caller: Caller,
        _data: SearchTransactionsRequest,
        _rpc_caller: RpcCaller,
    ) -> MentatResponse<SearchTransactionsResponse> {
        todo!()
    }
}
