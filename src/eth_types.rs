pub use ethereum_types::{Address, Secret, H256, U256};

// type for code
pub type Code = Vec<u8>;

pub trait FromStr {
    fn from_str(code_str: &str) -> Self;
}

impl FromStr for Code {
    fn from_str(code_str: &str) -> Self {
        let mut code = Code::new();

        let mut idx: usize = 0;
        let bytes = code_str.as_bytes();

        while idx < bytes.len() - 1 {
            let byte_part_a = bytes[idx] - b'0';
            let byte_part_b = bytes[idx + 1] - b'0';
            code.push(num_op::u8s_to_u8(byte_part_a, byte_part_b));
            idx += 2;
        }

        code
    }
}

pub mod num_op {
    pub fn u8s_to_u8(a: u8, b: u8) -> u8 {
        (a << 4) | b
    }
}
