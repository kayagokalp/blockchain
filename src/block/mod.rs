use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub mod block_data;
use block_data::BlockData;

#[derive(Debug, Clone)]
pub struct Block {
    pub hash: u64,
    pub block_data: BlockData,
}

impl Block {
    #[allow(dead_code)]
    pub fn new(block_data: BlockData) -> Self {
        Block {
            hash: Self::hash(&block_data),
            block_data,
        }
    }

    #[allow(dead_code)]
    pub fn new_with_data(index: u64, text: String, prev_hash: u64, nonce: u64) -> Self {
        let new_block_data = BlockData::new(index, text, prev_hash, nonce);
        Block {
            hash: Self::hash(&new_block_data),
            block_data: new_block_data,
        }
    }

    #[allow(dead_code)]
    pub fn set_nonce(&mut self, nonce: u64) {
        self.block_data.nonce = nonce;
        self.hash = Self::hash(&self.block_data)
    }

    #[allow(dead_code)]
    pub fn hash(block_data: &BlockData) -> u64 {
        let mut hasher = DefaultHasher::new();
        block_data.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use super::*;

    #[test]
    fn test_block_hash() {
        let block_data = BlockData::new(0, "dummy".to_string(), 0, 0);
        let block = Block::new(block_data);
        let mut hasher = DefaultHasher::new();
        block.block_data.hash(&mut hasher);
        let desired_hash = hasher.finish();
        assert_eq!(block.hash, desired_hash);
    }
    #[test]
    fn test_set_nonce() {
        let block_data = BlockData::new(0, "dummy".to_string(), 0, 0);
        let mut block = Block::new(block_data);
        let random_nonce: u64 = rand::random();
        block.set_nonce(random_nonce);

        assert_eq!(random_nonce, block.block_data.nonce);
    }
}
