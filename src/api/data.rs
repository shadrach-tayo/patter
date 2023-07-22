use serde::{Deserialize};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PinnedObject {
    pub ipfs_hash: String,
    pub pin_size: u64,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PinByFile {
    pub(crate) files: Vec<String>,
}

impl PinByFile {
    pub fn new<S: Into<String>>(path: S) -> Self {
        PinByFile { files: vec![path.into()] }
    }
}
