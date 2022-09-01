//! rosetta mempool api implementation for bitcoin using mentat

use super::*;

/// rosetta data routes for bitcoin
#[derive(Clone, Default)]
pub struct BitcoinMempoolApi;

#[async_trait]
impl MempoolApiRouter for BitcoinMempoolApi {}

#[async_trait]
impl MempoolApi for BitcoinMempoolApi {
    async fn mempool(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        rpc_caller: RpcCaller,
    ) -> Result<MempoolResponse> {
        let transaction_identifiers = rpc_caller
            .rpc_call::<Response<Vec<String>>>(BitcoinJrpc::new("getrawmempool", &[] as &[u8]))
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
        rpc_caller: RpcCaller,
    ) -> Result<MempoolTransactionResponse> {
        let tx_hash = trim_hash(&data.transaction_identifier.hash);
        let mempool = rpc_caller
            .rpc_call::<Response<Vec<String>>>(BitcoinJrpc::new("getrawmempool", &[] as &[u8]))
            .await?;

        if let Some((i, _)) = mempool
            .into_iter()
            .enumerate()
            .find(|(_, id)| id.as_str() == tx_hash)
        {
            let transaction = rpc_caller
                .rpc_call::<Response<BitcoinTransaction>>(BitcoinJrpc::new(
                    "getrawtransaction",
                    &[json!(tx_hash), json!(true)],
                ))
                .await?
                .into_transaction(i, &rpc_caller)
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
