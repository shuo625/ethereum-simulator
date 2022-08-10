use super::{eth_types::H256, hash, tx::Tx};

pub struct Block {
    tx: Tx,
    tx_hash: H256,
    prev_block_hash: H256,
}

impl Block {
    pub fn new(tx: Tx, prev_block_hash: H256) -> Self {
        let tx_hash = tx.hash();
        Block {
            tx,
            tx_hash: tx_hash,
            prev_block_hash,
        }
    }

    pub fn hash(&self) -> H256 {
        hash::keccak(format!("{}{}", self.tx_hash, self.prev_block_hash))
    }
}
