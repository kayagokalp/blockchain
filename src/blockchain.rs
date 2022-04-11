#[path = "block/mod.rs"]
pub mod block;
use block::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let zero_addr: block::block_data::transaction::Address = 0u64;

        let first_transaction_data: block::block_data::transaction::TransactionData =
            block::block_data::transaction::TransactionData::new(zero_addr, zero_addr, 100000);

        let first_transaction: block::block_data::transaction::Transaction =
            block::block_data::transaction::Transaction::new(first_transaction_data);
        let mut genesis_block: Block = Block::new_with_data(0, "genesis_bebek".to_string(), 0, 0);
        genesis_block.block_data.add_transaction(first_transaction);

        println!(
            "Genesis block generated!, block hash: {}, block nonce: {} ",
            genesis_block.hash, genesis_block.block_data.nonce
        );
        Self::mine_block(&mut genesis_block);
        println!(
            "Geneis block mined!, block hash: {}, block nonce: {} ",
            genesis_block.hash, genesis_block.block_data.nonce
        );
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    fn mine_block(current_block: &mut Block) -> u64 {
        let mut current_nonce = 0;
        let mut solved = false;
        while !solved {
            current_block.set_nonce(current_nonce);
            solved = Self::is_solved(current_block.hash);
            if !solved {
                current_nonce += 1;
            }
        }
        current_nonce
    }

    pub fn add_block(&mut self, block_to_add: &mut Block) -> u64 {
        block_to_add.block_data.index = self.blocks.last().unwrap().block_data.index + 1;
        block_to_add.block_data.prev_hash = self.blocks.last().unwrap().hash;
        let mined_nonce = Self::mine_block(block_to_add);
        self.blocks.push(block_to_add.clone());
        mined_nonce
    }

    fn is_solved(current_hash: u64) -> bool {
        //checks if the first *1* byte is 0.
        let mask = 0b1111111100000000000000000000000000000000000000000000000000000000;
        current_hash & mask == 0
    }

    pub fn print_blockchain(&self) {
        for block in self.blocks.iter() {
            println!(
                "Block {}, hash {}, prev_hash {}",
                block.block_data.index, block.hash, block.block_data.prev_hash
            );
            for transaction in block.block_data.transactions.iter() {
                println!(
                    "--- Transaction tx: {} timestamp {}, from {} to {} value {}",
                    transaction.tx,
                    transaction.transaction_data.timestamp,
                    transaction.transaction_data.from,
                    transaction.transaction_data.recipient,
                    transaction.transaction_data.value
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Blockchain;

    #[test]
    fn test_is_solved() {
        let true_value = 1u64;
        let false_value = 0b1111000000000000000000000000000000000000000000000000000000000000u64;
        assert_eq!(Blockchain::is_solved(true_value), true);
        assert_eq!(Blockchain::is_solved(false_value), false);
    }
    #[test]
    fn test_mine_block() {
        let mut block = Block::new_with_data(0, "dummy".to_string(), 0, 0);
        Blockchain::mine_block(&mut block);
        assert_eq!(Blockchain::is_solved(block.hash), true);
    }
    #[test]
    fn test_add_block() {
        let mut blockchain = Blockchain::new();
        let mut block_item = Block::new_with_data(0, "dummy".to_string(), 0, 0);
        blockchain.add_block(&mut block_item);
        assert_eq!(blockchain.blocks[1].block_data.index, 1);
        assert_eq!(
            blockchain.blocks[1].block_data.prev_hash,
            blockchain.blocks[0].hash
        );
    }
}
