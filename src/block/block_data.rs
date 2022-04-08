extern crate chrono;
use chrono::{DateTime, Utc};

#[derive(Debug, Hash, PartialEq)]
pub struct BlockData {
    pub index: u64,
    pub text: String,
    pub timestamp: DateTime<Utc>,
    pub prev_hash: u64,
    pub nonce: u64,
}

impl BlockData {
    #[allow(dead_code)]
    pub fn new(index: u64, text: String, prev_hash: u64, nonce: u64) -> Self {
        BlockData {
            index,
            text,
            timestamp: Utc::now(),
            prev_hash,
            nonce,
        }
    }
}
