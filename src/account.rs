mod storage;

pub use self::storage::Storage;
use crate::{cli::cmd_errors::CmdTxErrCode, eth_types::H256, hash::keccak};

pub enum AccountType {
    EoA,
    Contract,
}

pub struct Account {
    name: String,
    account_type: AccountType,
    private_key: String,
    address: String,
    balance: u64,
    code_hash: H256,
    code: String,
    storage: Storage,
}

impl Account {
    pub fn new(name: String, code: String) -> Self {
        //let private_key = PrivateKey::from_raw(&[22]).expect("create private key failed");
        //let address = private_key.public();

        Account {
            name,
            account_type: if code == "" {
                AccountType::EoA
            } else {
                AccountType::Contract
            },
            private_key: String::new(),
            address: String::new(),
            balance: 100,
            code_hash: keccak(&code),
            code,
            storage: Storage::new(),
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
