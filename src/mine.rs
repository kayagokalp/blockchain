use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[allow(dead_code)]
pub fn mine_block(hash: u64) -> u64 {
    let mut nonce = 0;
    let mut solved = false;

    while !solved {
        let current_hash = hash_current(hash + nonce);
        solved = is_solved(current_hash);
        if !solved {
            nonce += 1;
        }
    }
    nonce
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
    let mask = 15u64 << 60;
    println!("flags: {:#064b}", mask);
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
        let dummy_data = "kaya";
        let mut hasher = DefaultHasher::new();
        dummy_data.hash(&mut hasher);
        let dummy_data_hash = hasher.finish();
        let calculated_nonce = mine_block(dummy_data_hash);
        assert_eq!(
            is_solved(hash_current(dummy_data_hash + calculated_nonce)),
            true
        )
    }
}
