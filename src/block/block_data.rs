extern crate chrono;
use chrono::{DateTime, Utc};

#[path = "../transaction.rs"]
mod transaction;
use transaction::Transaction;

#[derive(Debug)]
pub struct BlockData {
    pub index: u64,
    pub transactions: Vec<Transaction>,
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
            transactions: Vec::new(),
            text,
            timestamp: Utc::now(),
            prev_hash,
            nonce,
        }
    }
    #[allow(dead_code)]
    pub fn add_transaction(&mut self, transaction_to_add: &Transaction) {
        self.transactions.push(*transaction_to_add);
    }
}

impl std::hash::Hash for BlockData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.text.hash(state);
        self.timestamp.hash(state);
        self.prev_hash.hash(state);
        self.nonce.hash(state);
        for transaction in self.transactions.iter() {
            transaction.tx.hash(state)
        }
    }
}

impl std::cmp::PartialEq for BlockData {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
            && self.transactions == other.transactions
            && self.text == other.text
            && self.timestamp == other.timestamp
            && self.prev_hash == other.prev_hash
            && self.nonce == other.nonce
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    use transaction::TransactionData;
    #[test]
    fn test_add_transaction() {
        let mut dummy_block_data = BlockData::new(0, "dummy_text".to_string(), 0, 0);

        let dummy_from = 0x00000000;
        let dummy_rec = 0x00000000;
        let dummy_val = 10;

        let dummy_transaction_data = TransactionData::new(dummy_from, dummy_rec, dummy_val);
        let dummy_transaction = Transaction::new(dummy_transaction_data);

        dummy_block_data.add_transaction(&dummy_transaction);

        assert_eq!(dummy_block_data.transactions.len(), 1);
        assert_eq!(dummy_block_data.transactions[0], dummy_transaction);
    }
}
