//! a bitcoind getblock response

use futures::future::join_all;
use mentat::{
    api::MentatResponse, axum::Json, errors::*, identifiers::BlockIdentifier, models::Block,
    responses::BlockResponse, server::RpcCaller,
};

use super::*;
use crate::responses::common::BitcoinTransaction;

/// a bitcoind getblock response
#[allow(non_snake_case, clippy::missing_docs_in_private_items)]
#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct GetBlockResponse {
    pub hash: String,
    // confirmations: usize,
    pub height: u64,
    pub version: usize,
    // versionHex: String,
    pub merkleroot: String,
    pub time: u64,
    pub mediantime: u64,
    pub nonce: usize,
    pub bits: String,
    pub difficulty: f64,
    // chainwork: String,
    // nTx: usize,
    pub previousblockhash: String,
    // nextblockhash: String,
    // strippedsize: usize,
    pub size: usize,
    pub weight: usize,
    pub tx: Vec<BitcoinTransaction>,
}

impl GetBlockResponse {
    /// convert GetBlock into a rosetta `BlockResponse`.
    /// makes calls to the bitcoind node during the conversion to get transaction info
    pub async fn into_block_response(
        self,
        rpc_caller: &RpcCaller,
    ) -> MentatResponse<BlockResponse> {
        Ok(Json(BlockResponse {
            block: Some(Block {
                transactions: join_all(
                    self.tx
                        .into_iter()
                        .enumerate()
                        .map(|(i, tx)| tx.into_transaction(i, rpc_caller)),
                )
                .await
                .into_iter()
                .collect::<Result<_, _>>()?,
                block_identifier: BlockIdentifier {
                    index: self.height,
                    hash: self.hash,
                },
                parent_block_identifier: BlockIdentifier {
                    index: self.height.saturating_sub(1),
                    hash: self.previousblockhash,
                },
                timestamp: self.time * 1000,
                metadata: [
                    ("bits".to_string(), self.bits.into()),
                    ("difficulty".to_string(), self.difficulty.into()),
                    ("mediantime".to_string(), self.mediantime.into()),
                    ("merkleroot".to_string(), self.merkleroot.into()),
                    ("nonce".to_string(), self.nonce.into()),
                    ("size".to_string(), self.size.into()),
                    ("version".to_string(), self.version.into()),
                    ("weight".to_string(), self.weight.into()),
                ]
                .into(),
            }),
            other_transactions: None,
        }))
    }
}
