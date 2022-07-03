use std::collections::HashMap;

use crate::{account::EoA, block::Block, cli::cmd_errors::CmdTxErrCode, tx::Tx};

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

    pub fn tx_send(&mut self, from: &str, to: &str, value: u64) -> Result<(), CmdTxErrCode> {
        self.txs
            .push(Tx::new(from.to_string(), to.to_string(), value));
        self.mine()
    }

    fn mine(&mut self) -> Result<(), CmdTxErrCode> {
        let last_tx = self.txs.last().unwrap();

        if let Some(from_account) = self.accounts.get_mut(last_tx.from()) {
            // if from exists
            match from_account.sub_balance(last_tx.value()) {
                Ok(_) => {
                    // balance of from is enough
                    if let Some(to_account) = self.accounts.get_mut(last_tx.to()) {
                        // if to exists
                        to_account.add_balance(last_tx.value());
                        // mine tx
                        if self.blocks.is_empty() {
                            self.blocks.push(Block::new(
                                last_tx.clone(),
                                String::from("0000000000000000000000000000000000000000000000000000000000000000"),
                            ));
                        } else {
                            self.blocks.push(Block::new(
                                last_tx.clone(),
                                self.blocks.last().unwrap().hash(),
                            ))
                        }
                        Ok(())
                    } else {
                        // to does not exist
                        self.txs.pop();
                        Err(CmdTxErrCode::NOTEXISTEDTO)
                    }
                }
                err => {
                    // balance of from is not enough
                    self.txs.pop();
                    err
                }
            }
        } else {
            // from does not exist
            self.txs.pop();
            Err(CmdTxErrCode::NOTEXISTEDFROM)
        }
    }
}
