//! bitcoind fields used in responses

use mentat::serde::Deserialize;

mod bitcoin_transaction;
pub use bitcoin_transaction::*;

mod fee_estimate;
pub use fee_estimate::*;