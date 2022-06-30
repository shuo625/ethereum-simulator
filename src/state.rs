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

    pub fn account_add(&mut self, name: String) {
        let name_copy = name.clone();
        self.accounts.insert(name, EoA::new(name_copy));
    }

    pub fn account_list(&self) -> String {
        let mut rst = String::new();
        for key in self.accounts.keys() {
            rst.push_str(key)
        }

        rst
    }

    pub fn account_get_balance(&self, name: &String) -> u64 {
        self.accounts.get(name).unwrap().get_balance()
    }

    pub fn tx_send(&mut self, args: String) {
        todo!()
    }
}
