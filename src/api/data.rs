use serde::{Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PinnedObject {
    pub ipfs_hash: String,
    pub pin_size: u64,
    pub timestamp: String,
}
