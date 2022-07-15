use crate::{
    cli::cmd_errors::CmdTxErrCode,
    eth_types::{Bytes, H256},
    hash::keccak,
};

pub struct Account {
    name: String,
    private_key: String,
    address: String,
    balance: u64,
    code_hash: H256,
    code: Bytes,
}

impl Account {
    pub fn new(name: String, code: Bytes) -> Self {
        //let private_key = PrivateKey::from_raw(&[22]).expect("create private key failed");
        //let address = private_key.public();

        Account {
            name,
            private_key: String::new(),
            address: String::new(),
            balance: 100,
            code_hash: keccak(&code),
            code,
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
