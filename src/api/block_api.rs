//! rosetta block api implementation for bitcoin using mentat

use super::*;

/// rosetta data routes for bitcoin
#[derive(Clone, Default)]
pub struct BitcoinBlockApi;

#[async_trait]
impl BlockApiRouter for BitcoinBlockApi {}

#[async_trait]
impl BlockApi for BitcoinBlockApi {
    async fn block(
        &self,
        _caller: Caller,
        data: BlockRequest,
        rpc_caller: RpcCaller,
    ) -> Result<BlockResponse> {
        let hash = if let Some(block_hash) = data.block_identifier.hash {
            trim_hash(&block_hash).to_string()
        } else if let Some(block_id) = data.block_identifier.index {
            rpc_caller
                .rpc_call::<Response<String>>(BitcoinJrpc::new("getblockhash", &[block_id]))
                .await?
        } else {
            rpc_caller
                .rpc_call::<Response<String>>(BitcoinJrpc::new("getbestblockhash", &[] as &[u8]))
                .await?
        };

        rpc_caller
            .rpc_call::<Response<GetBlockResponse>>(BitcoinJrpc::new(
                "getblock",
                &[json!(hash), json!(2)],
            ))
            .await?
            .into_block_response(&rpc_caller)
            .await
    }

    async fn block_transaction(
        &self,
        _caller: Caller,
        data: BlockTransactionRequest,
        rpc_caller: RpcCaller,
    ) -> Result<BlockTransactionResponse> {
        let block_hash = trim_hash(&data.block_identifier.hash);
        let tx_hash = trim_hash(&data.transaction_identifier.hash);

        let block = rpc_caller
            .rpc_call::<Response<GetBlockResponse>>(BitcoinJrpc::new(
                "getblock",
                &[json!(block_hash), json!(2u32)],
            ))
            .await?;

        if let Some((i, tx)) = block
            .tx
            .into_iter()
            .enumerate()
            .find(|(_, tx)| tx.hash == tx_hash)
        {
            Ok(BlockTransactionResponse {
                transaction: tx.into_transaction(i, &rpc_caller).await?,
            })
        } else {
            MentatError::transaction_not_found(Some(&data.transaction_identifier.hash))
        }
    }
}
