use std::ops::Add;

use crate::eth_types::Address;

pub struct Tx {
    from: Address,
    to: Address,
    value: u64,
}

impl Tx {
    pub fn new(from: Address, to: Address, value: u64) -> Self {
        Tx { from, to, value }
    }
}
