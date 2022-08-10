use super::{
    eth_types::{Address, Bytes, H256},
    hash,
};

pub struct Tx {
    from: Address,
    to: Address,
    value: u64,
    data: Bytes,
    gasprice: u64,
}

impl Tx {
    pub fn new(from: Address, to: Address, value: u64, data: Bytes) -> Self {
        Tx {
            from,
            to,
            value,
            data,
            gasprice: 10,
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

    pub fn gasprice(&self) -> u64 {
        self.gasprice
    }

    pub fn hash(&self) -> H256 {
        hash::keccak(format!(
            "{}{}{}",
            &self.from,
            &self.to,
            self.value.to_string(),
        ))
    }
}
