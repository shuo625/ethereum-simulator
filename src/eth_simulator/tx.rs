use super::{
    eth_types::{Address, Bytes, H256},
    hash,
};

#[derive(Clone, Copy)]
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
    contract_name: String,
}

impl Tx {
    pub fn new(
        from: Address,
        to: Address,
        value: usize,
        data: Bytes,
        tx_type: TxType,
        contract_name: String,
    ) -> Self {
        Tx {
            from,
            to,
            value,
            data,
            gasprice: 10,
            tx_type,
            contract_name,
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

    pub fn tx_type(&self) -> TxType {
        self.tx_type
    }

    pub fn contract_name(&self) -> Option<&str> {
        match self.tx_type {
            TxType::DeployContract => Some(&self.contract_name),
            _ => None,
        }
    }
}
