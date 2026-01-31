use blake3::Hasher;
use types::transaction::{Transaction, TxHash};

pub fn tx_hash(txn: &Transaction) -> TxHash {
    let bytes = serde_json::to_vec(txn).expect("Transaction must be serializable");
    let mut h = Hasher::new();
    h.update(&bytes);
    h.finalize().to_hex().to_string()
}
