use std::collections::HashMap;

use crate::eth_types::H256;

pub struct Storage {
    storage: HashMap<H256, H256>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            storage: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: H256, value: H256) {
        self.storage.insert(key, value);
    }

    pub fn get(&self, key: &H256) -> H256 {
        self.storage.get(key).unwrap().clone()
    }
}