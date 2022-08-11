use std::str::FromStr;

use ethereum_types::BigEndianHash;
pub use ethereum_types::{Address, Secret, H256, U256};

// type for code
pub type Code = Vec<u8>;
pub type Bytes = Vec<u8>;

/// Custom EthFrom<T> trait instead of that from std lib for implementing for external types
/// Make EthFrom as a unified type conversion between eth types
pub trait EthFrom<T> {
    fn ethfrom(obj: T) -> Self;
}

impl EthFrom<Address> for U256 {
    fn ethfrom(obj: Address) -> Self {
        H256::from(obj).into_uint()
    }
}

impl EthFrom<&Address> for U256 {
    fn ethfrom(obj: &Address) -> Self {
        let addr = obj.clone();
        H256::from(addr).into_uint()
    }
}

impl EthFrom<H256> for U256 {
    fn ethfrom(obj: H256) -> Self {
        obj.into_uint()
    }
}

impl EthFrom<bool> for U256 {
    fn ethfrom(obj: bool) -> Self {
        if obj {
            U256::one()
        } else {
            U256::zero()
        }
    }
}

impl EthFrom<u64> for U256 {
    fn ethfrom(obj: u64) -> Self {
        U256::from(obj)
    }
}

impl EthFrom<usize> for U256 {
    fn ethfrom(obj: usize) -> Self {
        U256::from(obj)
    }
}

impl EthFrom<&[u8]> for U256 {
    fn ethfrom(obj: &[u8]) -> Self {
        U256::from_big_endian(obj)
    }
}

impl EthFrom<U256> for Address {
    fn ethfrom(obj: U256) -> Self {
        let addr: H256 = BigEndianHash::from_uint(&obj);
        Address::from(addr)
    }
}

impl EthFrom<&str> for Address {
    fn ethfrom(obj: &str) -> Self {
        if obj.len() == 0 {
            Address::zero()
        } else {
            Address::from_str(obj).unwrap()
        }
    }
}

impl EthFrom<&String> for Address {
    fn ethfrom(obj: &String) -> Self {
        if obj.len() == 0 {
            Address::zero()
        } else {
            Address::from_str(obj).unwrap()
        }
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

impl EthFrom<U256> for H256 {
    fn ethfrom(obj: U256) -> Self {
        H256::from_uint(&obj)
    }
}

pub trait EthSign {
    fn to_sign(self) -> Self;
    fn is_neg(&self) -> bool;
}

impl EthSign for U256 {
    fn to_sign(self) -> Self {
        let U256(arr) = self;
        let sign = arr[3].leading_zeros() == 0;

        if sign {
            (!U256::zero() ^ self).overflowing_add(U256::one()).0
        } else {
            self
        }
    }

    fn is_neg(&self) -> bool {
        let U256(arr) = self;
        arr[3].leading_zeros() == 0
    }
}

pub mod num_op {
    pub fn u8s_to_u8(a: u8, b: u8) -> u8 {
        (a << 4) | b
    }
}
