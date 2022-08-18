use super::{eth_types::H256, hash, tx::Tx};

#[allow(dead_code)]
pub struct Block {
    tx: Tx,
    block_hash: H256,
    prev_block_hash: H256,
}

impl Block {
    pub fn new(tx: Tx, prev_block_hash: H256) -> Self {
        let tx_hash = tx.hash();
        Block {
            tx,
            block_hash: hash::keccak(format!("{}{}", tx_hash, prev_block_hash)),
            prev_block_hash,
        }
    }

    pub fn get_hash(&self) -> H256 {
        self.block_hash
    }
}
