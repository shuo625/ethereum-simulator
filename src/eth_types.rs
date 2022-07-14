use uint::construct_uint;

pub type Address = ethsign::PublicKey;
pub type PrivateKey = ethsign::SecretKey;

construct_uint! {
    pub struct U256(4);
}
