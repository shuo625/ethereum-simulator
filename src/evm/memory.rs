use crate::eth_types::U256;

pub struct Memory {
    memory: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Memory { memory: Vec::new() }
    }
}
