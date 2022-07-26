//! The optional api endpoints for bitcoind.

use mentat::{
    api::{Caller, CallerOptionalApi, MentatResponse, OptionalApi},
    axum::{async_trait, Json},
    conf::{Mode, NodePid},
    server::RpcCaller,
    sysinfo::Pid,
    types::{HealthCheckResponse, NodeConnections, NodeNetwork, Result, Synced},
};

use crate::{
    request::BitcoinJrpc,
    responses::{
        data::{GetBlockchainInfoResponse, GetNetworkInfo},
        *,
    },
};

/// The optional api endpoints for bitcoind.
#[derive(Clone, Default)]
pub struct BitcoinOptionalApi;

// TODO: this is a clunky design pattern
#[async_trait]
impl CallerOptionalApi for BitcoinOptionalApi {
    async fn call_health(
        &self,
        caller: Caller,
        mode: &Mode,
        rpc_caller: RpcCaller,
        server_pid: Pid,
        node_pid: NodePid,
    ) -> MentatResponse<HealthCheckResponse> {
        self.health(caller, mode, rpc_caller, server_pid, node_pid)
            .await
    }
}

#[async_trait]
impl OptionalApi for BitcoinOptionalApi {
    async fn synced(&self, rpc_caller: RpcCaller) -> MentatResponse<Synced> {
        let result = rpc_caller
            .rpc_call::<Response<GetBlockchainInfoResponse>>(BitcoinJrpc::new(
                "getblockchaininfo",
                &[] as &[()],
            ))
            .await?;

        Ok(Json(Synced {
            local_tip: result.blocks,
            global_tip: result.headers,
        }))
    }

    async fn node_address(&self, rpc_caller: &RpcCaller) -> Result<String> {
        let result = rpc_caller
            .rpc_call::<Response<Vec<Address>>>(BitcoinJrpc::new("getnodeaddresses", &[] as &[()]))
            .await?;

        Ok(result[0].address.clone())
    }

    async fn node_connections(
        &self,
        mode: &Mode,
        rpc_caller: &RpcCaller,
    ) -> Result<Option<NodeConnections>> {
        if mode.is_offline() {
            Ok(Some(NodeConnections::Offline))
        } else {
            let result = rpc_caller
                .rpc_call::<Response<GetNetworkInfo>>(BitcoinJrpc::new(
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
        rpc_caller: &RpcCaller,
    ) -> Result<Option<NodeNetwork>> {
        if mode.is_offline() {
            Ok(Some(NodeNetwork::Offline))
        } else {
            let result = rpc_caller
                .rpc_call::<Response<Network>>(BitcoinJrpc::new("getnettotals", &[] as &[()]))
                .await?;

            Ok(Some(NodeNetwork::Online {
                bytes_recv: result.totalbytesrecv,
                bytes_sent: result.totalbytessent,
            }))
        }
    }
}
