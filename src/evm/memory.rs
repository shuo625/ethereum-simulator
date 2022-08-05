use crate::eth_types::U256;

pub struct Memory {
    memory: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Memory { memory: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.memory.len()
    }

    pub fn read(&self, offset: U256) -> U256 {
        let off = offset.low_u64() as usize;
        U256::from(&self.memory[off..off + 32])
    }

    pub fn read_slice(&self, offset: U256, size: U256) -> &[u8] {
        let off = offset.low_u64() as usize;
        let len = size.low_u64() as usize;

        &self.memory[off..off + len]
    }

    pub fn write(&mut self, offset: U256, value: U256) {
        let off = offset.low_u64() as usize;
        value.to_big_endian(&mut self.memory[off..off + 32]);
    }

    pub fn write_slice(&mut self, offset: U256, value: &[u8]) {
        let off = offset.low_u64() as usize;
        self.memory[off..off + value.len()].copy_from_slice(value);
    }

    pub fn write_byte(&mut self, offset: U256, value: U256) {
        let off = offset.low_u64() as usize;
        let val = value.low_u64() as u8;
        self.memory[off] = val;
    }
}
