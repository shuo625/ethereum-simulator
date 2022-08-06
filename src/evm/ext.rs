use crate::{account::Storage, eth_types::H256, tx::Tx};

pub struct Ext<'a> {
    storage: &'a mut Storage,
    tx: &'a Tx,
    chainid: usize,
}

impl<'a> Ext<'a> {
    pub fn new(storage: &'a mut Storage, tx: &'a Tx) -> Self {
        Ext {
            storage,
            tx,
            chainid: 0,
        }
    }

    pub fn set(&mut self, key: H256, value: H256) {
        self.storage.set(key, value);
    }

    pub fn get(&self, key: &H256) -> H256 {
        self.storage.get(key)
    }
}
