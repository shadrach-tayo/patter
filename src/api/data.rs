use serde::{Deserialize};
use serde_derive::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    /// Pinata is running preliminary validations on your pin request.
    Prechecking,
    /// Pinata is actively searching for your content on the IPFS network.
    Searching,
    /// Pinata has located your content and is now in the process of retrieving it.
    Retrieving,
    /// Pinata wasn't able to find your content after a day of searching the IPFS network.
    Expired,
    /// Pinning this object would put you over the free tier limit. Please add a credit card
    /// to continue.
    OverFreeLimit,
    /// This object is too large of an item to pin. If you're seeing this, please contact pinata
    /// for a more custom solution.
    OverMaxSize,
    /// The object you're attempting to pin isn't readable by IPFS nodes.
    InvalidObject,
    /// You provided a host node that was either invalid or unreachable.
    BadHostNode,
}

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

// impl PinByFile {
//     pub fn new<S: Into<String>>(path: S) -> Self {
//         PinByFile { files: vec![path.into()] }
//     }
// }

#[derive(Debug, Deserialize, Clone)]
pub struct PinByJson {
    pub(crate) file: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PinByHash {
    pub(crate) hash_to_pin: String,
}

// impl PinByJson {
//     pub fn new<S: Into<String>>(path: S) -> Self {
//         PinByJson { file: path.into() }
//     }
// }

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PinByHashResult {
    pub id: String,
    pub ipfs_hash: String,
    pub status: JobStatus,
    pub name: Option<String>,
}
