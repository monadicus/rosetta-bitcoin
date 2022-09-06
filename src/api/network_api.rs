//! rosetta network api implementation for bitcoin using mentat

use super::*;

/// rosetta data routes for bitcoin
#[derive(Clone, Default)]
pub struct BitcoinNetworkApi;

#[async_trait]
impl NetworkApiRouter for BitcoinNetworkApi {}

#[async_trait]
impl NetworkApi for BitcoinNetworkApi {
    type NodeCaller = BitcoinCaller;

    async fn network_list(
        &self,
        _caller: Caller,
        _data: MetadataRequest,
        node_caller: &Self::NodeCaller,
    ) -> Result<NetworkListResponse> {
        Ok(node_caller
            .rpc_call::<GetBlockchainInfoResponse>(BitcoinJrpc::new(
                "getblockchaininfo",
                &[] as &[u8],
            ))
            .await?
            .into())
    }

    // TODO: this can be quite general for all mentat implementations
    async fn network_options(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        node_caller: &Self::NodeCaller,
    ) -> Result<NetworkOptionsResponse> {
        Ok(NetworkOptionsResponse {
            version: Version {
                // TODO: fetch this
                // This is just the current Rosetta version for now
                rosetta_version: "1.4.12".to_owned(),
                node_version: node_caller
                    .rpc_call::<GetNetworkInfo>(BitcoinJrpc::new("getnetworkinfo", &[] as &[u8]))
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
                call_methods: Vec::new(),
                balance_exemptions: Vec::new(),
                mempool_coins: false,
            },
        })
    }

    async fn network_status(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        node_caller: &Self::NodeCaller,
    ) -> Result<NetworkStatusResponse> {
        let current_hash = node_caller
            .rpc_call::<String>(BitcoinJrpc::new("getbestblockhash", &[] as &[u8]))
            .await?;
        let current_block = node_caller
            .rpc_call::<GetBlockResponse>(BitcoinJrpc::new(
                "getblock",
                &[json!(current_hash), json!(2)],
            ))
            .await?;

        let genesis_hash = node_caller
            .rpc_call::<String>(BitcoinJrpc::new("getblockhash", &[0]))
            .await?;
        let genesis_block = node_caller
            .rpc_call::<GetBlockResponse>(BitcoinJrpc::new(
                "getblock",
                &[json!(genesis_hash), json!(2)],
            ))
            .await?;

        Ok(NetworkStatusResponse {
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
            peers: node_caller
                .rpc_call::<Vec<PeerInfo>>(BitcoinJrpc::new("getpeerinfo", &[] as &[u8]))
                .await?
                .into_iter()
                .map(|p| p.into())
                .collect(),
        })
    }
}
