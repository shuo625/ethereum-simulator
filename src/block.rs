use sha256;

use crate::tx::Tx;

pub struct Block {
    tx: Tx,
    tx_hash: String,
    prev_block_hash: String,
}

impl Block {
    pub fn new(tx: Tx, prev_block_hash: String) -> Self {
        let tx_hash = tx.hash();
        Block {
            tx,
            tx_hash: tx_hash,
            prev_block_hash,
        }
    }

    pub fn hash(&self) -> String {
        sha256::digest(format!("{}{}", self.tx_hash, self.prev_block_hash))
    }
}
