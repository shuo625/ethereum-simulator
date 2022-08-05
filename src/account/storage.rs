use crate::eth_types::H256;
use std::collections::HashMap;

pub struct Storage {
    storage: HashMap<H256, H256>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            storage: HashMap::new(),
        }
    }
}
