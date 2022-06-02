//! The optional endpoint payloads.

use mentat::serde::Deserialize;

/// The node address.
#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct Address {
    /// The address string.
    pub address: String,
}

/// The node network data usage.
#[derive(Debug, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct Network {
    /// The bytes received.
    pub totalbytesrecv: u64,
    /// The bytes sent.
    pub totalbytessent: u64,
}
