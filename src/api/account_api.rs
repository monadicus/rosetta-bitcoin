//! rosetta account api implementation for bitcoin using mentat

use super::*;

/// rosetta data routes for bitcoin
#[derive(Clone, Default)]
pub struct BitcoinAccountApi;

#[async_trait]
impl AccountApiRouter for BitcoinAccountApi {}

#[async_trait]
impl AccountApi for BitcoinAccountApi {
    async fn account_balance(
        &self,
        _caller: Caller,
        data: AccountBalanceRequest,
        rpc_caller: RpcCaller,
    ) -> Result<AccountBalanceResponse> {
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
                    .height as usize,
                hash,
            }),
            _ => None,
        };

        let args = if let Some(id) = &id {
            vec![
                json!("start"),
                json!(vec!(json!(ScanObjectsDescriptor {
                    desc: format!("addr({})", trim_hash(&data.account_identifier.address)),
                    range: id.index as i64,
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

        Ok(rpc_caller
            .rpc_call::<Response<ScanTxOutSetResult>>(BitcoinJrpc::new("scantxoutset", &args))
            .await?
            .into_balance(id))
    }
}
