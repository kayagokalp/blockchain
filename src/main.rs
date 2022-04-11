mod blockchain;
use blockchain::block::Block;
use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    let mut next_block = Block::new_with_data(0, "".to_string(), 0, 0);
    blockchain.add_block(&mut next_block);
    blockchain.print_blockchain();
}
