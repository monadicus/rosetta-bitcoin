//! rosetta data api implementation for bitcoind using mentat

use mentat::{
    api::{Caller, CallerDataApi, DataApi, MentatResponse},
    axum::{async_trait, Json},
    errors::*,
    identifiers::{BlockIdentifier, PartialBlockIdentifier, TransactionIdentifier},
    indexmap::IndexMap,
    misc::{OperationStatus, Version},
    models::Allow,
    requests::*,
    responses::*,
    serde_json::json,
    server::RpcCaller,
};

use crate::{
    request::{trim_hash, BitcoinJrpc, ScanObjectsDescriptor},
    responses::{common::BitcoinTransaction, data::*, Response},
};

/// rosetta data routes for bitcoind
#[derive(Clone, Default)]
pub struct BitcoinDataApi;

#[async_trait]
impl CallerDataApi for BitcoinDataApi {}

#[async_trait]
impl DataApi for BitcoinDataApi {
    async fn network_list(
        &self,
        _caller: Caller,
        _data: MetadataRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<NetworkListResponse> {
        Ok(Json(
            rpc_caller
                .rpc_call::<Response<GetBlockchainInfoResponse>>(BitcoinJrpc::new(
                    "getblockchaininfo",
                    &[] as &[u8],
                ))
                .await?
                .into(),
        ))
    }

    // TODO: this can be quite general for all mentat implementations
    async fn network_options(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<NetworkOptionsResponse> {
        Ok(Json(NetworkOptionsResponse {
            version: Version {
                // TODO: fetch this
                // This is just the current Rosetta version for now
                rosetta_version: "1.4.10".to_owned(),
                node_version: rpc_caller
                    .rpc_call::<Response<GetNetworkInfo>>(BitcoinJrpc::new(
                        "getnetworkinfo",
                        &[] as &[u8],
                    ))
                    .await?
                    .version()
                    .to_string(),
                middleware_version: Some(env!("CARGO_PKG_VERSION").to_owned()),
                metadata: IndexMap::new(),
            },
            allow: Allow {
                operation_statuses: vec![
                    OperationStatus {
                        status: "SUCCESS".to_owned(),
                        successful: true,
                    },
                    OperationStatus {
                        status: "SKIPPED".to_owned(),
                        successful: false,
                    },
                ],
                operation_types: vec![
                    "INPUT".to_owned(),
                    "OUTPUT".to_owned(),
                    "COINBASE".to_owned(),
                ],
                errors: MentatError::all_errors(),
                historical_balance_lookup: true,
                timestamp_start_index: None,
                // TODO: populate this when `/call` is populated.
                call_methods: None,
                balance_exemptions: None,
                mempool_coins: false,
            },
        }))
    }

    async fn network_status(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<NetworkStatusResponse> {
        let current_hash = rpc_caller
            .rpc_call::<Response<String>>(BitcoinJrpc::new("getbestblockhash", &[] as &[u8]))
            .await?;
        let current_block = rpc_caller
            .rpc_call::<Response<GetBlockResponse>>(BitcoinJrpc::new(
                "getblock",
                &[json!(current_hash), json!(2)],
            ))
            .await?;

        let genesis_hash = rpc_caller
            .rpc_call::<Response<String>>(BitcoinJrpc::new("getblockhash", &[0]))
            .await?;
        let genesis_block = rpc_caller
            .rpc_call::<Response<GetBlockResponse>>(BitcoinJrpc::new(
                "getblock",
                &[json!(genesis_hash), json!(2)],
            ))
            .await?;

        Ok(Json(NetworkStatusResponse {
            current_block_identifier: BlockIdentifier {
                index: current_block.height,
                hash: current_hash,
            },
            current_block_timestamp: current_block.time,
            genesis_block_identifier: BlockIdentifier {
                index: genesis_block.height,
                hash: genesis_hash,
            },
            oldest_block_identifier: None,
            sync_status: None,
            peers: rpc_caller
                .rpc_call::<Response<Vec<PeerInfo>>>(BitcoinJrpc::new("getpeerinfo", &[] as &[u8]))
                .await?
                .into_iter()
                .map(|p| p.into())
                .collect(),
        }))
    }

    async fn account_balance(
        &self,
        _caller: Caller,
        data: AccountBalanceRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<AccountBalanceResponse> {
        let id = match data.block_identifier {
            Some(PartialBlockIdentifier {
                index: Some(index),
                hash: Some(hash),
            }) => Some(BlockIdentifier { index, hash }),
            Some(PartialBlockIdentifier {
                index: Some(index),
                hash: None,
            }) => Some(BlockIdentifier {
                index,
                hash: rpc_caller
                    .rpc_call::<Response<String>>(BitcoinJrpc::new("getblockhash", &[index]))
                    .await?,
            }),
            Some(PartialBlockIdentifier {
                index: None,
                hash: Some(hash),
            }) => Some(BlockIdentifier {
                index: rpc_caller
                    .rpc_call::<Response<GetBlockResponse>>(BitcoinJrpc::new(
                        "getblock",
                        &[json!(trim_hash(&hash)), json!(2u32)],
                    ))
                    .await?
                    .height,
                hash,
            }),
            _ => None,
        };

        let args = if let Some(id) = &id {
            vec![
                json!("start"),
                json!(vec!(json!(ScanObjectsDescriptor {
                    desc: format!("addr({})", trim_hash(&data.account_identifier.address)),
                    range: id.index,
                }))),
            ]
        } else {
            vec![
                json!("start"),
                json!(vec!(json!(format!(
                    "addr({})",
                    trim_hash(&data.account_identifier.address)
                )))),
            ]
        };

        Ok(Json(
            rpc_caller
                .rpc_call::<Response<ScanTxOutSetResult>>(BitcoinJrpc::new("scantxoutset", &args))
                .await?
                .into_balance(id),
        ))
    }

    async fn block(
        &self,
        _caller: Caller,
        data: BlockRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<BlockResponse> {
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
    ) -> MentatResponse<BlockTransactionResponse> {
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
            Ok(Json(BlockTransactionResponse {
                transaction: tx.into_transaction(i, &rpc_caller).await?,
            }))
        } else {
            MentatError::transaction_not_found(Some(&data.transaction_identifier.hash))
        }
    }

    async fn mempool(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<MempoolResponse> {
        let transaction_identifiers = rpc_caller
            .rpc_call::<Response<Vec<String>>>(BitcoinJrpc::new("getrawmempool", &[] as &[u8]))
            .await?
            .into_iter()
            .map(|hash| TransactionIdentifier { hash })
            .collect();
        Ok(Json(MempoolResponse {
            transaction_identifiers,
        }))
    }

    async fn mempool_transaction(
        &self,
        _caller: Caller,
        data: MempoolTransactionRequest,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<MempoolTransactionResponse> {
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
            Ok(Json(MempoolTransactionResponse {
                transaction,
                metadata: IndexMap::new(),
            }))
        } else {
            MentatError::transaction_not_found(Some(&data.transaction_identifier.hash))
        }
    }
}
