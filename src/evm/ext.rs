use crate::{
    account::Storage,
    eth_types::{H256, U256},
    tx::Tx,
};

pub struct Ext<'a> {
    storage: &'a mut Storage,
    tx: &'a Tx,
    chainid: usize,
    gas: usize,
}

impl<'a> Ext<'a> {
    pub fn new(storage: &'a mut Storage, tx: &'a Tx) -> Self {
        Ext {
            storage,
            tx,
            chainid: 0,
            gas: 100,
        }
    }

    pub fn set_storage(&mut self, key: H256, value: H256) {
        self.storage.set(key, value);
    }

    pub fn get_storage(&self, key: &H256) -> H256 {
        self.storage.get(key)
    }

    pub fn get_gas(&self) -> U256 {
        U256::from(self.gas)
    }

    pub fn get_chainid(&self) -> U256 {
        U256::from(self.chainid)
    }
}
