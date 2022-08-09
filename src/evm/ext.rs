use std::collections::HashMap;

use crate::{
    account::Account,
    eth_types::{Address, EthFrom, H256, U256},
    tx::Tx,
};

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

    pub fn get_storage(&self, key: &H256) -> H256 {
        self.accounts.get(&self.account).unwrap().get_storage(key)
    }

    pub fn get_gas(&self) -> U256 {
        U256::from(self.gas)
    }

    pub fn get_chainid(&self) -> U256 {
        U256::from(self.chainid)
    }

    pub fn get_callvalue(&self) -> U256 {
        let value = self.tx.value();
        U256::from(value)
    }

    pub fn get_address(&self) -> U256 {
        U256::ethfrom(&self.account)
    }

    pub fn get_balance(&self, address: &Address) -> U256 {
        U256::from(self.accounts.get(address).unwrap().get_balance())
    }
}
