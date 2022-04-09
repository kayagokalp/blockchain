use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

extern crate chrono;
use chrono::{DateTime, Utc};
type Address = u64;

#[derive(Debug)]
pub struct Transaction {
    pub transaction_data: TransactionData,
    pub tx: u64,
}

// Should i implement Copy and Clone here? Not so sure..
// TODO(kaya): Return here! This piece of code should be implemented better!
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct TransactionData {
    pub recipient: Address,
    pub from: Address,
    pub value: u64,
    pub timestamp: DateTime<Utc>,
}

impl Transaction {
    #[allow(dead_code)]
    pub fn new(in_transaction_data: TransactionData) -> Self {
        Transaction {
            transaction_data: in_transaction_data,
            tx: in_transaction_data.hash_transaction(),
        }
    }
}

impl TransactionData {
    #[allow(dead_code)]
    pub fn new(rec: Address, frm: Address, val: u64) -> Self {
        TransactionData {
            recipient: rec,
            from: frm,
            value: val,
            timestamp: Utc::now(),
        }
    }

    #[allow(dead_code)]
    pub fn hash_transaction(self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.tx == other.tx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Since each transaction is created with a transaction_data which includes current time_stamp they all going to be different
    // even though we give the same parameters. Their hashes must be different in that case!
    #[test]
    fn test_transaction_new() {
        let dummy_from = 0x00000000;
        let dummy_rec = 0x00000000;
        let dummy_val = 10;

        let dummy_transaction_data = TransactionData::new(dummy_from, dummy_rec, dummy_val);
        let dummy_transaction_data_2 = TransactionData::new(dummy_from, dummy_rec, dummy_val);

        let dummy_transaction = Transaction::new(dummy_transaction_data);
        let dummy_transaction_2 = Transaction::new(dummy_transaction_data_2);

        assert_ne!(dummy_transaction, dummy_transaction_2);
    }

    // Since each transaction_data is created with the current time_stamp they all going to be different
    // even though we give the same parameters. Their hashes must be different in that case!
    #[test]
    fn test_transaction_data_new() {
        let dummy_from = 0x00000000;
        let dummy_rec = 0x00000000;
        let dummy_val = 10;

        let dummy_transaction_data = TransactionData::new(dummy_from, dummy_rec, dummy_val);
        let dummy_transaction_data_2 = TransactionData::new(dummy_from, dummy_rec, dummy_val);

        assert_ne!(dummy_transaction_data, dummy_transaction_data_2);
    }
}
