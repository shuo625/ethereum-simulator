use fixed_hash::construct_fixed_hash;
use uint::construct_uint;

pub type Address = ethsign::PublicKey;
pub type PrivateKey = ethsign::SecretKey;

construct_uint! {
    pub struct U256(4);
}

construct_fixed_hash! {
    pub struct H256(32);
}
