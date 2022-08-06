pub use ethereum_types::{Address, Secret, H256, U256};

pub type Bytes = Vec<u8>;

pub mod num_op {
    pub fn u8s_to_u8(a: u8, b: u8) -> u8 {
        (a << 4) | b
    }
}
