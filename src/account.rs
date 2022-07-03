// use crate::eth_types::{Address, PrivateKey};
use crate::cli::cmd_errors::CmdTxErrCode;

pub struct EoA {
    name: String,
    private_key: String,
    address: String,
    balance: u64,
}

impl EoA {
    pub fn new(name: String) -> Self {
        //let private_key = PrivateKey::from_raw(&[22]).expect("create private key failed");
        //let address = private_key.public();

        EoA {
            name,
            private_key: String::new(),
            address: String::new(),
            balance: 100,
        }
    }

    pub fn get_balance(&self) -> u64 {
        self.balance
    }

    pub fn add_balance(&mut self, value: u64) {
        self.balance += value;
    }

    pub fn sub_balance(&mut self, value: u64) -> Result<(), CmdTxErrCode> {
        if self.balance >= value {
            self.balance -= value;
            Ok(())
        } else {
            Err(CmdTxErrCode::NOTENOUGHBALANCE)
        }
    }
}
