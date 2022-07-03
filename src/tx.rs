use sha256;

// use crate::eth_types::Address;

pub struct Tx {
    from: String,
    to: String,
    value: u64,
}

impl Tx {
    pub fn new(from: String, to: String, value: u64) -> Self {
        Tx { from, to, value }
    }

    pub fn from(&self) -> &str {
        &self.from
    }

    pub fn to(&self) -> &str {
        &self.to
    }

    pub fn value(&self) -> u64 {
        self.value
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
