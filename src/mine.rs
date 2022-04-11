use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
#[path = "./block/mod.rs"]
mod block;
use block::Block;

#[allow(dead_code)]
pub fn mine_block_from_block(current_block: &mut Block) -> u64 {
    let mut current_nonce = 0;
    let mut solved = false;
    while !solved {
        current_block.set_nonce(current_nonce);
        solved = is_solved(current_block.hash);
        if !solved {
            current_nonce += 1;
        }
    }
    current_nonce
}

#[allow(dead_code)]
fn hash_current(current_data: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    current_data.hash(&mut hasher);
    hasher.finish()
}

#[allow(dead_code)]
pub fn is_solved(current_hash: u64) -> bool {
    //checks if the first *1* byte is 0.
    let mask = 0b1111111100000000000000000000000000000000000000000000000000000000;
    current_hash & mask == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_solved() {
        let true_value = 1u64;
        let false_value = 0b1111000000000000000000000000000000000000000000000000000000000000u64;
        assert_eq!(is_solved(true_value), true);
        assert_eq!(is_solved(false_value), false);
    }
    #[test]
    fn test_mine_block() {
        let mut block = Block::new_with_data(0, "dummy".to_string(), 0, 0);
        mine_block_from_block(&mut block);
        assert_eq!(is_solved(block.hash), true);
    }
}
