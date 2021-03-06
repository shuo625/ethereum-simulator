use sha256;

// use crate::eth_types::Address;

#[derive(Clone)]
pub struct Tx {
    from: String,
    to: String,
    value: u64,
    data: String,
}

impl Tx {
    pub fn new(from: String, to: String, value: u64, data: String) -> Self {
        Tx {
            from,
            to,
            value,
            data,
        }
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

    pub fn data(&self) -> &str {
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
