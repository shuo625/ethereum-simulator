use crate::{
    account::{Account, Storage},
    eth_types::{Address, H256, U256},
    state::State,
    tx::Tx,
};

pub struct Ext<'a> {
    state: &'a State,
    account: &'a Account,
    storage: &'a mut Storage,
    tx: &'a Tx,
    chainid: usize,
    gas: usize,
}

impl<'a> Ext<'a> {
    pub fn new(state: &'a State, account: &'a mut Account, tx: &'a Tx) -> Self {
        Ext {
            state,
            account,
            storage: account.get_mut_storage(),
            tx,
            chainid: 0,
            gas: 100,
        }
    }

    pub fn set_storage(&mut self, key: H256, value: H256) {
        self.storage.set(key, value);
    }

    pub fn get_storage(&self, key: &H256) -> H256 {
        self.storage.get(key)
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
        U256::from(self.account.get_address().as_bytes())
    }

    pub fn get_balance(&self, address: Address) -> U256 {
        U256::from(
            self.state
                .account_get_balance(&address.to_string())
                .unwrap(),
        )
    }
}
