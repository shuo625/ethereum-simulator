pub use ethereum_types::{Address, Secret, H256, U256};

// type for code
pub type Code = Vec<u8>;
pub type Bytes = Vec<u8>;

/// Custom EthFrom<T> trait instead of that from std lib for implementing for external types
pub trait EthFrom<T> {
    fn ethfrom(obj: T) -> Self;
}

impl EthFrom<Address> for U256 {
    fn ethfrom(obj: Address) -> Self {
        U256::from_big_endian(obj.as_bytes())
    }
}

impl EthFrom<&Address> for U256 {
    fn ethfrom(obj: &Address) -> Self {
        U256::from_big_endian(obj.as_bytes())
    }
}

impl EthFrom<H256> for U256 {
    fn ethfrom(obj: H256) -> Self {
        U256::from_big_endian(obj.as_bytes())
    }
}

impl EthFrom<U256> for Address {
    fn ethfrom(obj: U256) -> Self {
        let mut s = Bytes::new();
        obj.to_big_endian(s.as_mut_slice());
        Address::from_slice(s.as_slice())
    }
}

impl EthFrom<&str> for Vec<u8> {
    fn ethfrom(obj: &str) -> Self {
        let mut v: Vec<u8> = Vec::new();

        let mut idx: usize = 0;
        let bytes = obj.as_bytes();

        while idx < bytes.len() - 1 {
            let byte_part_a = bytes[idx] - b'0';
            let byte_part_b = bytes[idx + 1] - b'0';
            v.push(num_op::u8s_to_u8(byte_part_a, byte_part_b));
            idx += 2;
        }

        v
    }
}

pub trait EthConvert {
    fn to_sign(obj: Self) -> Self;
}

impl EthConvert for U256 {
    fn to_sign(obj: Self) -> Self {
        let U256(arr) = obj;
        let sign = arr[3].leading_zeros() == 0;

        if sign {
            (!U256::zero() ^ obj).overflowing_add(U256::one()).0
        } else {
            obj
        }
    }
}

pub mod num_op {
    pub fn u8s_to_u8(a: u8, b: u8) -> u8 {
        (a << 4) | b
    }
}
