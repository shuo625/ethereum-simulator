mod storage;

use self::storage::Storage;
use super::{
    eth_types::{Address, Code, Secret, H256},
    hash::keccak,
};

pub enum AccountError {
    NotEnoughBalance,
    NotExistedStorageKey,
}

pub enum AccountType {
    EoA,
    Contract,
}

pub struct Account {
    name: String,
    account_type: AccountType,
    private_key: Secret,
    address: Address,
    balance: u64,
    code_hash: H256,
    code: Code,
    storage: Storage,
}

impl Account {
    pub fn new(name: String, code: Code) -> Self {
        Account {
            name,
            account_type: if code.len() == 0 {
                AccountType::EoA
            } else {
                AccountType::Contract
            },
            private_key: Secret::random(),
            address: Address::random(),
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

    pub fn sub_balance(&mut self, value: u64) -> Result<(), AccountError> {
        if self.balance >= value {
            self.balance -= value;
            Ok(())
        } else {
            Err(AccountError::NotEnoughBalance)
        }
    }

    pub fn get_address(&self) -> &Address {
        &self.address
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_code(&self) -> &Code {
        &self.code
    }

    pub fn set_storage(&mut self, key: H256, value: H256) {
        self.storage.set(key, value);
    }

    pub fn get_storage(&self, key: &H256) -> Result<H256, AccountError> {
        match self.storage.get(key) {
            Some(v) => Ok(v),
            None => Err(AccountError::NotExistedStorageKey),
        }
    }
}