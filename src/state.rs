use serde_json::{self, Value};

use std::{collections::HashMap, fs::File, io::BufReader};

use crate::{account::Account, block::Block, cli::cmd_errors::CmdTxErrCode, evm::VM, tx::Tx};

pub struct State {
    accounts: HashMap<String, Account>,
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
        self.accounts.insert(
            name.to_string(),
            Account::new(name.to_string(), "".to_string()),
        );
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

    pub fn tx_send(&mut self, params_file: &str) -> Result<(), CmdTxErrCode> {
        let params: Value =
            serde_json::from_reader(BufReader::new(File::open(params_file).unwrap())).unwrap();
        let tx = Tx::new(
            params["from"].to_string(),
            params["to"].to_string(),
            params["value"].to_string().parse::<u64>().unwrap(),
            params["data"].to_string(),
        );
        self.txs.push(tx);
        self.mine()
    }

    fn handle_tx(&self, tx: &Tx) {
        if tx.to() == "" {
            self.handle_tx_deploy_contract(tx);
        } else if tx.data() == "" {
            self.handle_tx_eoa_to_eoa(tx);
        } else {
            self.handle_tx_call_contract(tx);
        }
    }

    fn handle_tx_eoa_to_eoa(&self, tx: &Tx) {}

    fn handle_tx_deploy_contract(&self, tx: &Tx) {
        let vm = VM::new(tx.data());
    }

    fn handle_tx_call_contract(&self, tx: &Tx) {}

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
