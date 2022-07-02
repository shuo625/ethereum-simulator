use std::collections::HashMap;

use crate::{account::EoA, block::Block, tx::Tx};

pub struct State {
    accounts: HashMap<String, EoA>,
    blocks: Vec<Block>,
    txs: Vec<Tx>,
}

impl State {
    pub fn new() -> Self {
        State {
            accounts: HashMap::new(),
            blocks: Vec::new(),
            txs: Vec::new(),
        }
    }

    pub fn account_add(&mut self, name: &str) {
        self.accounts
            .insert(name.to_string(), EoA::new(name.to_string()));
    }

    pub fn account_list(&self) -> Vec<&String> {
        self.accounts.keys().collect()
    }

    pub fn account_get_balance(&self, name: &str) -> Option<u64> {
        match self.accounts.get(name) {
            Some(account) => Some(account.get_balance()),
            None => None,
        }
    }

    pub fn tx_send(&mut self, args: String) {
        todo!()
    }
}
