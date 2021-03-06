//! a bitcoind scantxoutsetresult response

use mentat::{
    identifiers::BlockIdentifier,
    indexmap::IndexMap,
    models::{Amount, Currency},
    responses::AccountBalanceResponse,
    serde::Deserialize,
};

// #[derive(Clone, Debug, Deserialize)]
// #[serde(crate = "mentat::serde")]
// pub struct Unspents {
//     txid: String,
//     vout: usize,
//     scriptPubKey: String,
//     desc: String,
//     amount: f64,
//     height: usize,
// }

/// a bitcoind scantxoutsetresult response
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct ScanTxOutSetResult {
    // success: bool,
    // txouts: usize,
    height: u64,
    bestblock: String,
    // unspents: Vec<Unspents>,
    total_amount: f64,
}

impl ScanTxOutSetResult {
    /// convert scantxoutset response into account/balance response
    pub fn into_balance(self, id: Option<BlockIdentifier>) -> AccountBalanceResponse {
        AccountBalanceResponse {
            block_identifier: id.unwrap_or(BlockIdentifier {
                index: self.height,
                hash: self.bestblock,
            }),
            balances: vec![Amount {
                value: (self.total_amount * 100000000.0).to_string(),
                currency: Currency {
                    symbol: String::from("BTC"),
                    decimals: 8,
                    metadata: IndexMap::new(),
                },
                metadata: IndexMap::new(),
            }],
            metadata: IndexMap::new(),
        }
    }
}
