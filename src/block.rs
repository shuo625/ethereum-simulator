use sha256;

use crate::tx::Tx;

pub struct Block {
    tx_hash: String,
    prev_block_hash: String,
}

impl Block {
    pub fn mine(tx: Tx, prev_block_hash: String) -> Self {
        Block {
            tx_hash: tx.hash(),
            prev_block_hash,
        }
    }

    pub fn hash(&self) -> String {
        sha256::digest(format!("{}{}", self.tx_hash, self.prev_block_hash))
    }
}
