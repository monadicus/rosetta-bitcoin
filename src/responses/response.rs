//! a bitcoind response

use std::fmt::Debug;

use mentat::{
    serde::{de::DeserializeOwned, Deserialize},
    server::RpcResponse,
    tracing,
    types::Result,
};

use super::ErrorResponse;
use crate::request::BitcoinJrpc;

/// the response structure for bitcoind
#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct Response<R> {
    /// a successful bitcoind response
    pub result: Option<R>,
    /// a bitcoind error response
    pub error: Option<ErrorResponse>,
}

impl<O> RpcResponse for Response<O>
where
    O: Debug + DeserializeOwned,
{
    type I = BitcoinJrpc;
    type O = O;

    fn unwrap_response(self) -> Result<Self::O> {
        tracing::debug!("res: {self:#?}");
        match self {
            Response {
                result: Some(res),
                error: None,
            } => Ok(res),
            Response {
                result: None,
                error: Some(err),
            } => err.into(),
            _ => Err(format!("unknown response: {self:?}").into()),
        }
    }
}
