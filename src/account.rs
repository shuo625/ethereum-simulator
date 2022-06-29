use ethsign::{PublicKey, SecretKey};

pub struct EoA {
    name: String,
    private_key: SecretKey,
    address: PublicKey,
    balance: u64,
}

impl EoA {
    pub fn new(name: &str) -> Self {
        let private_key = SecretKey::from_raw(&[22]).expect("create private key failed");
        let address = private_key.public();

        EoA {
            name: name.to_string(),
            private_key,
            address,
            balance: 100,
        }
    }
}
