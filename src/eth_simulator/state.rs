use std::collections::HashMap;

use super::{
    account::Account,
    block::Block,
    eth_types::{Address, Bytes, Code, EthFrom, H256},
    evm::{Ext, VMError, VMResult, VM},
    tx::{Tx, TxType},
};

#[derive(Debug)]
pub enum StateError {
    NotExistedAddress(Address),
    NotEnoughBalance,
    VMError(VMError),
    CallEoAAccount,
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

    pub fn account_add(&mut self, name: &str) -> Address {
        self.account_add_inner(name, Code::ethfrom(""))
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

    pub fn tx_send(&mut self, tx: Tx) -> Result<Option<Bytes>, StateError> {
        self.txs.push(tx);
        let last_tx = self.txs.last().unwrap().clone();

        match self.handle_tx(&last_tx) {
            Ok(result) => {
                self.mine(last_tx);
                Ok(result)
            }
            Err(err) => Err(err),
        }
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

    fn handle_tx(&mut self, tx: &Tx) -> Result<Option<Bytes>, StateError> {
        match tx.tx_type() {
            TxType::EoaToEoa => self.handle_tx_eoa_to_eoa(tx),
            TxType::DeployContract => self.handle_tx_deploy_contract(tx),
            TxType::CallContract => self.handle_tx_call_contract(tx),
        }
    }

    fn handle_tx_eoa_to_eoa(&mut self, tx: &Tx) -> Result<Option<Bytes>, StateError> {
        let from = self.get_mut_account_by_address(tx.from())?;

        match from.sub_balance(tx.value()) {
            Ok(_) => {
                let to = self.get_mut_account_by_address(tx.to())?;
                to.add_balance(tx.value());
                Ok(None)
            }
            Err(_) => Err(StateError::NotEnoughBalance),
        }
    }

    fn handle_tx_deploy_contract(&mut self, tx: &Tx) -> Result<Option<Bytes>, StateError> {
        let address = self.account_add_inner(tx.contract_name().unwrap(), tx.data().clone());
        let mut vm = VM::new(self.accounts.get(&address).unwrap().get_code().clone());
        let mut ext = Ext::new(address, &mut self.accounts, tx);

        match vm.execute(&mut ext) {
            Ok(vm_result) => match vm_result {
                VMResult::Ok | VMResult::Stop => Ok(None),
                VMResult::Return(bytes) => {
                    let account = self.get_mut_account_by_address(&address).unwrap();
                    account.set_code(bytes);
                    Ok(None)
                }
            },
            Err(err) => Err(StateError::VMError(err)),
        }
    }

    fn handle_tx_call_contract(&mut self, tx: &Tx) -> Result<Option<Bytes>, StateError> {
        let account = self.get_account_by_address(tx.to())?;
        if !account.is_contract() {
            return Err(StateError::CallEoAAccount);
        }

        let mut vm = VM::new(account.get_code().clone());
        let mut ext = Ext::new(account.get_address().clone(), &mut self.accounts, tx);

        match vm.execute(&mut ext) {
            Ok(vm_result) => match vm_result {
                VMResult::Ok | VMResult::Stop => Ok(None),
                VMResult::Return(bytes) => Ok(Some(bytes)),
            },
            Err(err) => Err(StateError::VMError(err)),
        }
    }

    fn mine(&mut self, last_tx: Tx) {
        let prev_block_hash = if self.blocks.len() == 0 {
            H256::zero()
        } else {
            self.blocks.last().unwrap().get_hash()
        };
        self.blocks.push(Block::new(last_tx, prev_block_hash));
    }
}
