//! The optional api endpoints for bitcoin.

use super::*;

/// The optional api endpoints for bitcoin.
#[derive(Clone, Default)]
pub struct BitcoinOptionalApi;

// TODO: this is a clunky design pattern
#[async_trait]
impl OptionalApiRouter for BitcoinOptionalApi {
    async fn call_health(
        &self,
        caller: Caller,
        mode: &Mode,
        node_caller: &Self::NodeCaller,
        server_pid: Pid,
        node_pid: NodePid,
    ) -> MentatResponse<HealthCheckResponse> {
        Ok(Json(
            self.health(caller, mode, node_caller, server_pid, node_pid)
                .await?,
        ))
    }
}

#[async_trait]
impl OptionalApi for BitcoinOptionalApi {
    type NodeCaller = BitcoinCaller;

    async fn synced(&self, node_caller: &Self::NodeCaller) -> Result<Synced> {
        let result = node_caller
            .rpc_call::<GetBlockchainInfoResponse>(BitcoinJrpc::new(
                "getblockchaininfo",
                &[] as &[()],
            ))
            .await?;

        Ok(Synced {
            local_tip: result.blocks,
            global_tip: result.headers,
        })
    }

    async fn node_address(&self, node_caller: &Self::NodeCaller) -> Result<String> {
        let result = node_caller
            .rpc_call::<Vec<Address>>(BitcoinJrpc::new(
                "getnodeaddresses",
                &[] as &[()],
            ))
            .await?;

        Ok(result[0].address.clone())
    }

    async fn node_connections(
        &self,
        mode: &Mode,
        node_caller: &Self::NodeCaller,
    ) -> Result<Option<NodeConnections>> {
        if mode.is_offline() {
            Ok(Some(NodeConnections::Offline))
        } else {
            let result = node_caller
                .rpc_call::<GetNetworkInfo>(BitcoinJrpc::new(
                    "getnetworkinfo",
                    &[] as &[()],
                ))
                .await?;

            Ok(Some(NodeConnections::Online {
                total: result.connections,
                inbound: result.connections_in,
                outbound: result.connections_out,
            }))
        }
    }

    async fn node_net_usage(
        &self,
        mode: &Mode,
        node_caller: &Self::NodeCaller,
    ) -> Result<Option<NodeNetwork>> {
        if mode.is_offline() {
            Ok(Some(NodeNetwork::Offline))
        } else {
            let result = node_caller
                .rpc_call::<Network>(BitcoinJrpc::new(
                    "getnettotals",
                    &[] as &[()],
                ))
                .await?;

            Ok(Some(NodeNetwork::Online {
                bytes_recv: result.totalbytesrecv,
                bytes_sent: result.totalbytessent,
            }))
        }
    }
}
