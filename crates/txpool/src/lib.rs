use std::collections::HashMap;
use types::transaction::{Transaction, TxHash};
pub struct TxPool {
    txs: HashMap<TxHash, Transaction>,
}

pub enum InsertionOutcome {
    Inserted(TxHash),
    AlreadyKnown(TxHash),
    Rejected(RejectReason),
}

pub enum RejectReason {
    PoolFull,
    FeeTooLow { min_fee: u64, got: u64 },
}

impl TxPool {
    pub fn new() -> Self {
        Self {
            txs: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.txs.len()
    }

    pub fn insert_tx(&mut self, hash: TxHash, txn: Transaction) -> InsertionOutcome {
        //Dedup
        if self.txs.contains_key(&hash) {
            return InsertionOutcome::AlreadyKnown(hash);
        }

        self.txs.insert(hash.clone(), txn);
        InsertionOutcome::Inserted(hash)
    }
}
