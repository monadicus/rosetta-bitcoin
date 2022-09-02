//! a bitcoind response

use std::fmt::Debug;

use mentat_server::{
    serde::{de::DeserializeOwned, Deserialize},
    tracing,
};
use mentat_types::Result;

use super::ErrorResponse;

/// the response structure for bitcoin
#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat_server::serde")]
pub struct BitcoinResponse<R> {
    /// a successful bitcoind response
    pub result: Option<R>,
    /// a bitcoind error response
    pub error: Option<ErrorResponse>,
}

impl<R> BitcoinResponse<R> {
    /// converts the server response into a result
    pub fn into_result(self) -> Result<R>
    where
        R: Debug + DeserializeOwned,
    {
        tracing::debug!("res: {self:?}");
        match self {
            BitcoinResponse {
                result: Some(res),
                error: None,
            } => Ok(res),
            BitcoinResponse {
                result: None,
                error: Some(err),
            } => err.into(),
            _ => Err(format!("unknown response: {self:?}").into()),
        }
    }
}
