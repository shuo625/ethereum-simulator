use std::collections::HashMap;

use super::super::{
    account::Account,
    eth_types::{Address, EthFrom, H256, U256},
    hash,
    tx::Tx,
};

pub enum ExtError {
    NotExistedAddress(Address),
    NotExistedStorageKey,
}

pub struct Ext<'a> {
    account: Address,
    accounts: &'a mut HashMap<Address, Account>,
    tx: &'a Tx,
    chainid: usize,
    gas: usize,
}

impl<'a> Ext<'a> {
    pub fn new(account: Address, accounts: &'a mut HashMap<Address, Account>, tx: &'a Tx) -> Self {
        Ext {
            account,
            accounts,
            tx,
            chainid: 0,
            gas: 100,
        }
    }

    pub fn set_storage(&mut self, key: H256, value: H256) {
        self.accounts
            .get_mut(&self.account)
            .unwrap()
            .set_storage(key, value);
    }

    pub fn get_storage(&self, key: &H256) -> Result<H256, ExtError> {
        match self.accounts.get(&self.account).unwrap().get_storage(key) {
            Ok(v) => Ok(v),
            Err(_) => Err(ExtError::NotExistedStorageKey),
        }
    }

    pub fn get_gas(&self) -> U256 {
        U256::ethfrom(self.gas)
    }

    pub fn get_chainid(&self) -> U256 {
        U256::ethfrom(self.chainid)
    }

    pub fn get_callvalue(&self) -> U256 {
        U256::ethfrom(self.tx.value())
    }

    pub fn get_address(&self) -> U256 {
        U256::ethfrom(&self.account)
    }

    pub fn get_balance(&self, address: &Address) -> Result<U256, ExtError> {
        self.get_account_and_then(address, |account| U256::ethfrom(account.get_balance()))
    }

    pub fn get_origin(&self) -> U256 {
        U256::ethfrom(self.tx.from())
    }

    pub fn get_caller(&self) -> U256 {
        U256::ethfrom(self.tx.from())
    }

    pub fn get_calldata(&self, i: U256) -> U256 {
        let idx = i.as_usize();
        U256::ethfrom(&self.tx.data()[idx..idx + 32])
    }

    pub fn get_calldatasize(&self) -> U256 {
        U256::ethfrom(self.tx.data().len())
    }

    pub fn get_calldata_slice(&self, offset: U256, length: U256) -> &[u8] {
        let off = offset.as_usize();
        let len = length.as_usize();
        &self.tx.data()[off..off + len]
    }

    pub fn get_codesize(&self) -> U256 {
        self.get_ext_codesize(&self.account).unwrap_unchecked()
    }

    pub fn get_code_slice(&self, offset: U256, length: U256) -> &[u8] {
        self.get_ext_code_slice(&self.account, offset, length)
            .unwrap_unchecked()
    }

    pub fn get_gasprice(&self) -> U256 {
        U256::ethfrom(self.tx.gasprice())
    }

    pub fn get_ext_codesize(&self, address: &Address) -> Result<U256, ExtError> {
        self.get_account_and_then(address, |account| U256::ethfrom(account.get_code().len()))
    }

    pub fn get_ext_code_slice(
        &self,
        address: &Address,
        offset: U256,
        length: U256,
    ) -> Result<&[u8], ExtError> {
        let off = offset.as_usize();
        let len = length.as_usize();

        match self.accounts.get(address) {
            Some(account) => Ok(&account.get_code()[off..off + len]),
            None => Err(ExtError::NotExistedAddress(address.clone())),
        }
    }

    pub fn get_ext_code_hash(&self, address: &Address) -> Result<U256, ExtError> {
        self.get_account_and_then(address, |account| {
            U256::ethfrom(hash::keccak(account.get_code()))
        })
    }

    fn get_account_and_then<T, F>(&self, address: &Address, f: F) -> Result<T, ExtError>
    where
        F: FnOnce(&Account) -> T,
    {
        match self.accounts.get(address) {
            Some(account) => Ok(f(account)),
            None => Err(ExtError::NotExistedAddress(address.clone())),
        }
    }
}
