use serde_json::{self, Value};

use std::{collections::HashMap, fs::File, io::BufReader};

use super::{
    account::Account,
    block::Block,
    eth_types::{Address, Bytes, Code, EthFrom, H256},
    evm::{Ext, VMError, VMResult, VM},
    tx::Tx,
};

pub enum StateError {
    NotExistedAddress(Address),
    NotEnoughBalance,
    VMError(VMError),
}

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

    pub fn account_add(&mut self, name: &str) {
        self.account_add_inner(name, Code::ethfrom(""));
    }

    pub fn account_list(&self) -> Vec<(&str, &Address, usize)> {
        let mut account_list: Vec<(&str, &Address, usize)> = Vec::new();

        for (k, v) in &self.accounts {
            account_list.push((v.get_name(), k, v.get_balance()));
        }

        account_list
    }

    pub fn account_get_balance(&self, address: &Address) -> Result<usize, StateError> {
        match self.accounts.get(address) {
            Some(account) => Ok(account.get_balance()),
            None => Err(StateError::NotExistedAddress(address.clone())),
        }
    }

    pub fn tx_send(&mut self, params_file: &str) -> Result<(), StateError> {
        let params: Value =
            serde_json::from_reader(BufReader::new(File::open(params_file).unwrap())).unwrap();
        let tx = Tx::new(
            Address::ethfrom(&params["from"].to_string()),
            Address::ethfrom(&params["to"].to_string()),
            params["value"].to_string().parse::<usize>().unwrap(),
            Bytes::ethfrom(&params["data"].to_string()),
        );

        self.txs.push(tx);
        self.mine()
    }

    fn account_add_inner(&mut self, name: &str, code: Code) -> Address {
        let account = Account::new(name.to_string(), code);
        let address = account.get_address().clone();
        self.accounts.insert(address.clone(), account);

        address
    }

    fn get_account_by_address(&self, address: &Address) -> Result<&Account, StateError> {
        self.accounts
            .get(address)
            .ok_or(StateError::NotExistedAddress(address.clone()))
    }

    fn get_mut_account_by_address(
        &mut self,
        address: &Address,
    ) -> Result<&mut Account, StateError> {
        self.accounts
            .get_mut(address)
            .ok_or(StateError::NotExistedAddress(address.clone()))
    }

    fn handle_tx(&mut self, tx: &Tx) -> Result<(), StateError> {
        if tx.to().is_zero() {
            self.handle_tx_deploy_contract(tx)
        } else if tx.data().len() == 0 {
            self.handle_tx_eoa_to_eoa(tx)
        } else {
            self.handle_tx_call_contract(tx)
        }
    }

    fn handle_tx_eoa_to_eoa(&mut self, tx: &Tx) -> Result<(), StateError> {
        let from = self.get_mut_account_by_address(tx.from())?;
        let to = self.get_mut_account_by_address(tx.to())?;

        match from.sub_balance(tx.value()) {
            Ok(_) => {
                to.add_balance(tx.value());
                Ok(())
            }
            Err(_) => Err(StateError::NotEnoughBalance),
        }
    }

    fn handle_tx_deploy_contract(&mut self, tx: &Tx) -> Result<(), StateError> {
        let address = self.account_add_inner("Contract", tx.data().clone());
        let account = self.get_account_by_address(&address).unwrap_unchecked();
        let mut vm = VM::new(account.get_code().clone());
        let mut ext = Ext::new(address, &mut self.accounts, tx);

        match vm.execute(&mut ext) {
            Ok(vm_result) => match vm_result {
                VMResult::Ok | VMResult::Stop => Ok(()),
                VMResult::Return(bytes) => {
                    account.set_code(bytes);
                    Ok(())
                }
            },
            Err(err) => Err(StateError::VMError(err)),
        }
    }

    fn handle_tx_call_contract(&self, tx: &Tx) -> Result<(), StateError> {
        Ok(())
    }

    fn mine(&mut self) -> Result<(), StateError> {
        let last_tx = self.txs.last().unwrap_unchecked().clone();

        match self.handle_tx(&last_tx) {
            Ok(_) => {
                let prev_block_hash = if self.blocks.len() == 0 {
                    H256::zero()
                } else {
                    self.blocks.last().unwrap_unchecked().get_hash()
                };
                self.blocks.push(Block::new(last_tx, prev_block_hash));
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}
