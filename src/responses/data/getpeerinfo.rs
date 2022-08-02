//! a bitcoind peerinfo field

use mentat_server::{indexmap::IndexMap, serde::Deserialize};
use mentat_types::Peer;

/// a bitcoind peer info field
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat_server::serde")]
pub struct PeerInfo {
    // id: usize,
    addr: String,
    // addrbind: String,
    // addrlocal: String,
    // network: String,
    // mapped_as: usize,
    // services: String,
    // servicesnames: Vec<String>,
    relaytxes: bool,
    lastsend: usize,
    lastrecv: usize,
    // last_transaction: usize,
    // last_block: usize,
    // bytessent: usize,
    // bytesrecv: usize,
    // conntime: usize,
    // timeoffset: usize,
    // pingtime: usize,
    // minping: usize,
    // pingwait: usize,
    version: usize,
    subver: String,
    // inbound: bool,
    // addnode: bool,
    // connection_type: String,
    startingheight: isize,
    banscore: Option<usize>,
    synced_headers: isize,
    synced_blocks: isize,
    // inflight: Vec<usize>,
    // whitelisted: bool,
    // permissions: Vec<String>,
    // minfeefilter: usize,
    // bytessent_per_msg: BytesPerMsg,
    // bytesrecv_per_msg: BytesPerMsg,
}

impl From<PeerInfo> for Peer {
    fn from(peer: PeerInfo) -> Self {
        Self {
            peer_id: peer.addr.clone(),
            metadata: {
                let mut map = IndexMap::new();
                map.insert("addr".to_string(), peer.addr.into());
                map.insert(
                    "banscore".to_string(),
                    peer.banscore.unwrap_or_default().into(),
                );
                map.extend([
                    ("lastrecv".to_string(), peer.lastrecv.into()),
                    ("lastsend".to_string(), peer.lastsend.into()),
                    ("relaytxes".to_string(), peer.relaytxes.into()),
                    ("startingheight".to_string(), peer.startingheight.into()),
                    ("subver".to_string(), peer.subver.into()),
                    ("synced_blocks".to_string(), peer.synced_blocks.into()),
                    ("synced_headers".to_string(), peer.synced_headers.into()),
                    ("version".to_string(), peer.version.into()),
                ]);
                map
            },
        }
    }
}
