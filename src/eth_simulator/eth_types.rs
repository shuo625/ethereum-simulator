use serde_json::Value;

use std::{fmt::Write, str::FromStr};

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

impl EthFrom<String> for Address {
    fn ethfrom(obj: String) -> Self {
        if obj.len() == 0 {
            Address::zero()
        } else {
            Address::from_str(&obj).unwrap()
        }
    }
}

impl EthFrom<&str> for Vec<u8> {
    fn ethfrom(obj: &str) -> Self {
        let mut v: Vec<u8> = Vec::new();

        let mut idx: usize = 0;
        let bytes = obj.as_bytes();

        if bytes.len() < 2 {
            return v;
        }

        while idx < bytes.len() - 1 {
            let byte_part_a = num_op::char_to_u8(bytes[idx]);
            let byte_part_b = num_op::char_to_u8(bytes[idx + 1]);
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

impl EthFrom<&Address> for String {
    fn ethfrom(obj: &Address) -> Self {
        let mut s = String::from("0x");
        for byte in obj.as_bytes() {
            write!(&mut s, "{:02x}", byte).unwrap();
        }

        s
    }
}

impl EthFrom<&Value> for String {
    fn ethfrom(obj: &Value) -> Self {
        obj.to_string()
            .strip_prefix('"')
            .unwrap()
            .strip_suffix('"')
            .unwrap()
            .to_string()
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

    pub fn char_to_u8(c: u8) -> u8 {
        if c >= b'0' && c <= b'9' {
            c - b'0'
        } else {
            c - b'a' + 10
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ethfrom_str_for_code() {
        use super::{Code, EthFrom};

        let bytes = "608060405234801561001057600080fd5b50610121806100206000396000f3fe6080604052348015600f57600080fd5b506004361060285760003560e01c8063ad59ac3b14602d575b600080fd5b60436004803603810190603f9190609d565b6057565b604051604e919060d2565b60405180910390f35b6000819050919050565b600080fd5b600063ffffffff82169050919050565b607d816066565b8114608757600080fd5b50565b6000813590506097816076565b92915050565b60006020828403121560b05760af6061565b5b600060bc84828501608a565b91505092915050565b60cc816066565b82525050565b600060208201905060e5600083018460c5565b9291505056fea2646970667358221220b420be65787785bfe7dcfb66b5a8df5ce0369da85c86b94dbc0e6cafb5fbae1c64736f6c63430008100033";
        let code = Code::ethfrom(bytes);
        let new_bytes: String = code.iter().map(|x| format!("{:02x}", x)).collect();

        assert_eq!(bytes, new_bytes);
    }
}
