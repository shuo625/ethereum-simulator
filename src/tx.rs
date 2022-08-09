use sha256;

use crate::eth_types::{Address, Bytes};

// use crate::eth_types::Address;

#[derive(Clone)]
pub struct Tx {
    from: Address,
    to: Address,
    value: u64,
    data: Bytes,
}

impl Tx {
    pub fn new(from: Address, to: Address, value: u64, data: Bytes) -> Self {
        Tx {
            from,
            to,
            value,
            data,
        }
    }

    pub fn from(&self) -> &Address {
        &self.from
    }

    pub fn to(&self) -> &Address {
        &self.to
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn data(&self) -> &Bytes {
        &self.data
    }

    pub fn hash(&self) -> String {
        sha256::digest(format!(
            "{}{}{}",
            &self.from,
            &self.to,
            self.value.to_string(),
        ))
    }
}
