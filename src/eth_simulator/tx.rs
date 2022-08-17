use super::{
    eth_types::{Address, Bytes, H256},
    hash,
};

#[derive(Clone)]
pub enum TxType {
    EoaToEoa,
    CallContract,
    DeployContract,
}

#[derive(Clone)]
pub struct Tx {
    from: Address,
    to: Address,
    value: usize,
    data: Bytes,
    gasprice: usize,
    tx_type: TxType,
}

impl Tx {
    pub fn new(from: Address, to: Address, value: usize, data: Bytes) -> Self {
        Tx {
            tx_type: if to.is_zero() {
                TxType::DeployContract
            } else if data.is_empty() {
                TxType::EoaToEoa
            } else {
                TxType::CallContract
            },
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

    pub fn value(&self) -> usize {
        self.value
    }

    pub fn data(&self) -> &Bytes {
        &self.data
    }

    pub fn gasprice(&self) -> usize {
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
