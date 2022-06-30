use sha256;

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

    pub fn hash(&self) -> String {
        sha256::digest(format!(
            "{}{}{}",
            String::from_utf8(self.from.bytes().to_vec()).unwrap(),
            String::from_utf8(self.to.bytes().to_vec()).unwrap(),
            self.value.to_string(),
        ))
    }
}
