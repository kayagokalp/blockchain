use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

mod block_data;
use block_data::BlockData;

#[derive(Debug)]
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
    fn hash(block_data: &BlockData) -> u64 {
        let mut hasher = DefaultHasher::new();
        block_data.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
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
}
