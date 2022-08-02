//! a bitcoind fee estimate field

use mentat_server::serde::Deserialize;

#[allow(clippy::missing_docs_in_private_items)]
#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat_server::serde")]
pub struct FeeEstimate {
    pub feerate: f64,
    pub errors: Vec<String>,
    pub blocks: usize,
}
