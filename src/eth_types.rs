use ethereum_types::BigEndianHash;
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

impl EthFrom<U256> for Address {
    fn ethfrom(obj: U256) -> Self {
        let addr: H256 = BigEndianHash::from_uint(&obj);
        Address::from(addr)
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
    fn to_sign(self) -> Self;
}

impl EthConvert for U256 {
    fn to_sign(self) -> Self {
        let U256(arr) = self;
        let sign = arr[3].leading_zeros() == 0;

        if sign {
            (!U256::zero() ^ self).overflowing_add(U256::one()).0
        } else {
            self
        }
    }
}

pub mod num_op {
    pub fn u8s_to_u8(a: u8, b: u8) -> u8 {
        (a << 4) | b
    }
}
