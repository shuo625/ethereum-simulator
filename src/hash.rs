use keccak_hash::write_keccak;

use crate::eth_types::H256;

pub fn keccak<T: AsRef<[u8]>>(s: T) -> H256 {
    let mut result = [0u8; 32];
    write_keccak(s, &mut result);
    H256(result)
}
