//! a bitcoind fee estimate field

use mentat::serde::Deserialize;

#[allow(clippy::missing_docs_in_private_items)]
#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct FeeEstimate {
    pub feerate: f64,
    pub errors: Vec<String>,
    pub blocks: usize,
}
