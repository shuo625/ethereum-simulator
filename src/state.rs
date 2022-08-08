use serde_json::{self, Value};

use std::{collections::HashMap, fs::File, io::BufReader, str::FromStr};

use crate::{
    account::Account,
    block::Block,
    cli::cmd_errors::CmdTxErrCode,
    eth_types::{Address, Code, EthFrom},
    evm::{Ext, VM},
    tx::Tx,
};

pub struct State {
    accounts: HashMap<Address, Account>,
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

    /// This is the external api used by cmd
    pub fn account_add(&mut self, name: &str) {
        self.account_add_inner(name, Code::ethfrom(""));
    }

    /// Return Vec<(name, address, balance)>
    pub fn account_list(&self) -> Vec<(&str, &Address, u64)> {
        let mut account_list: Vec<(&str, &Address, u64)> = Vec::new();

        for (k, v) in &self.accounts {
            account_list.push((v.get_name(), k, v.get_balance()));
        }

        account_list
    }

    pub fn account_get_balance(&self, address: &str) -> Option<u64> {
        let addr = Address::from_str(address).unwrap();
        match self.accounts.get(&addr) {
            Some(account) => Some(account.get_balance()),
            None => None,
        }
    }

    pub fn tx_send(&mut self, params_file: &str) -> Result<(), CmdTxErrCode> {
        let params: Value =
            serde_json::from_reader(BufReader::new(File::open(params_file).unwrap())).unwrap();
        let tx = Tx::new(
            Address::from_str(&params["from"].to_string()).unwrap(),
            Address::from_str(&params["to"].to_string()).unwrap(),
            params["value"].to_string().parse::<u64>().unwrap(),
            params["data"].to_string(),
        );
        self.handle_tx(&tx);
        self.txs.push(tx);
        self.mine()
    }

    fn account_add_inner(&mut self, name: &str, code: Code) -> Address {
        let account = Account::new(name.to_string(), code);
        let address = account.get_address().clone();
        self.accounts.insert(address.clone(), account);

        address
    }

    fn get_account_by_address(&self, address: Address) -> &Account {
        self.accounts.get(&address).unwrap()
    }

    fn handle_tx(&mut self, tx: &Tx) {
        if tx.to().to_string() == "" {
            self.handle_tx_deploy_contract(tx);
        } else if tx.data() == "" {
            self.handle_tx_eoa_to_eoa(tx);
        } else {
            self.handle_tx_call_contract(tx);
        }
    }

    fn handle_tx_eoa_to_eoa(&self, tx: &Tx) {}

    fn handle_tx_deploy_contract(&mut self, tx: &Tx) {
        let address = self.account_add_inner("Contract", Code::ethfrom(tx.data()));
        let account = self.get_account_by_address(address);
        let mut vm = VM::new(account.get_code());
        let mut ext = Ext::new(self, account, tx);
        vm.execute(&mut ext);
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
