extern crate chrono;
use chrono::{DateTime, Utc};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash,Hasher};



#[derive(Debug)]
pub struct Block {
    pub hash : u64,
    pub block_data : BlockData,
}

impl Block {
    pub fn new(block_data: BlockData) -> Self {
        Block {
            hash: Self::hash(&block_data),
            block_data,
        }
    }


    fn hash(block_data: &BlockData) -> u64 {
        let mut hasher = DefaultHasher::new();
        block_data.hash(&mut hasher);
        hasher.finish()
    }
}


