//! rosetta api implementations for bitcoin using mentat

mod account_api;
pub use account_api::*;
mod block_api;
pub use block_api::*;
mod call_api;
pub use call_api::*;
mod construction_api;
pub use construction_api::*;
mod events_api;
pub use events_api::*;
mod mempool_api;
pub use mempool_api::*;
mod network_api;
pub use network_api::*;
mod optional_api;
pub use optional_api::*;
mod search_api;
use std::str::FromStr;

use bitcoin::{
    hash_types::PubkeyHash,
    psbt::serialize::{Deserialize, Serialize},
    OutPoint, PackedLockTime, Script, Sequence, TxIn, TxOut, Txid, Witness,
};
use mentat_server::{
    api::{
        AccountApi, AccountApiRouter, BlockApi, BlockApiRouter, CallApi, CallApiRouter,
        ConstructionApi, ConstructionApiRouter, EventsApi, EventsApiRouter, MempoolApi,
        MempoolApiRouter, NetworkApi, NetworkApiRouter, OptionalApi, OptionalApiRouter, SearchApi,
        SearchApiRouter,
    },
    axum::{async_trait, Json},
    conf::{Mode, NodePid},
    indexmap::IndexMap,
    serde_json::{json, Value},
    sysinfo::Pid,
};
use mentat_types::*;
pub use search_api::*;

use crate::{
    request::{trim_hash, BitcoinCaller, BitcoinJrpc, ScanObjectsDescriptor},
    responses::{
        common::{BitcoinTransaction, FeeEstimate},
        data::{
            GetBlockResponse, GetBlockchainInfoResponse, GetNetworkInfo, PeerInfo,
            ScanTxOutSetResult,
        },
        Address, Network,
    },
};
