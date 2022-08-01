pub type Address = ethsign::PublicKey;
pub type PrivateKey = ethsign::SecretKey;

pub struct U256;
pub struct H256;

pub mod num_op {
    pub fn u8s_to_u8(a: u8, b: u8) -> u8 {
        (a << 4) | b
    }

    // TODO: consider generic type
    pub fn u8s_to_u16(nums: &[u8]) -> u16 {
        let value: u16 = 0;
        let len = nums.len();

        for (i, &num) in nums.iter().enumerate() {
            value = value | ((num as u16) << (len - 1 - i) * 8);
        }

        value
    }

    pub fn u8s_to_u32(nums: &[u8]) -> u32 {
        let value: u32 = 0;
        let len = nums.len();

        for (i, &num) in nums.iter().enumerate() {
            value = value | ((num as u32) << (len - 1 - i) * 8);
        }

        value
    }

    pub fn u8s_to_u64(nums: &[u8]) -> u64 {
        let value: u64 = 0;
        let len = nums.len();

        for (i, &num) in nums.iter().enumerate() {
            value = value | ((num as u64) << (len - 1 - i) * 8);
        }

        value
    }

    pub fn u8s_to_u128(nums: &[u8]) -> u128 {
        let value: u128 = 0;
        let len = nums.len();

        for (i, &num) in nums.iter().enumerate() {
            value = value | ((num as u128) << (len - 1 - i) * 8);
        }

        value
    }
}
