use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    #[serde(flatten)]
    inner: alloy::consensus::Header,
}
