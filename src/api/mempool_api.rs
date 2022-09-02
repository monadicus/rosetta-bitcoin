//! rosetta mempool api implementation for bitcoin using mentat

use super::*;

/// rosetta data routes for bitcoin
#[derive(Clone, Default)]
pub struct BitcoinMempoolApi;

#[async_trait]
impl MempoolApiRouter for BitcoinMempoolApi {}

#[async_trait]
impl MempoolApi for BitcoinMempoolApi {
    type NodeCaller = BitcoinCaller;

    async fn mempool(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        node_caller: &Self::NodeCaller,
    ) -> Result<MempoolResponse> {
        let transaction_identifiers = node_caller
            .rpc_call::<Vec<String>>(BitcoinJrpc::new("getrawmempool", &[] as &[u8]))
            .await?
            .into_iter()
            .map(|hash| TransactionIdentifier { hash })
            .collect();
        Ok(MempoolResponse {
            transaction_identifiers,
        })
    }

    async fn mempool_transaction(
        &self,
        _caller: Caller,
        data: MempoolTransactionRequest,
        node_caller: &Self::NodeCaller,
    ) -> Result<MempoolTransactionResponse> {
        let tx_hash = trim_hash(&data.transaction_identifier.hash);
        let mempool = node_caller
            .rpc_call::<Vec<String>>(BitcoinJrpc::new("getrawmempool", &[] as &[u8]))
            .await?;

        if let Some((i, _)) = mempool
            .into_iter()
            .enumerate()
            .find(|(_, id)| id.as_str() == tx_hash)
        {
            let transaction = node_caller
                .rpc_call::<BitcoinTransaction>(BitcoinJrpc::new(
                    "getrawtransaction",
                    &[json!(tx_hash), json!(true)],
                ))
                .await?
                .into_transaction(i, node_caller)
                .await?;
            Ok(MempoolTransactionResponse {
                transaction,
                metadata: IndexMap::new(),
            })
        } else {
            MentatError::transaction_not_found(Some(&data.transaction_identifier.hash))
        }
    }
}
