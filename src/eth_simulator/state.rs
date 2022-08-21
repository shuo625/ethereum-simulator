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
    TxError(TxError),
    VMError(VMError),
}

#[derive(Debug)]
pub enum TxError {
    NotEnoughBalance,
    CallEoAAccount,
    WrongFromAddress(Address),
    WrongToAddress(Address),
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

    pub fn address_exist(&self, address: &Address) -> bool {
        self.accounts.contains_key(address)
    }

    pub fn address_is_contract(&self, address: &Address) -> bool {
        self.accounts.get(address).unwrap().is_contract()
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

    pub fn account_get_balance(&self, address: &Address) -> Option<usize> {
        self.accounts
            .get(address)
            .and_then(|account| Some(account.get_balance()))
    }

    pub fn account_query_address_by_name(&self, name: &str) -> Option<Address> {
        for (address, account) in &self.accounts {
            if account.get_name() == name {
                return Some(address.clone());
            }
        }

        None
    }

    pub fn tx_send(&mut self, tx: Tx) -> Result<Bytes, StateError> {
        self.check_tx(&tx)
            .or_else(|tx_error| Err(StateError::TxError(tx_error)))?;

        self.txs.push(tx);
        let last_tx = self.txs.last().unwrap().clone();

        self.handle_tx(&last_tx).and_then(|result| {
            self.mine(last_tx);
            Ok(result)
        })
    }

    fn check_tx(&self, tx: &Tx) -> Result<(), TxError> {
        if !self.address_exist(tx.from()) {
            return Err(TxError::WrongFromAddress(tx.from().clone()));
        }
        if tx.tx_type() != TxType::DeployContract && !self.address_exist(tx.to()) {
            return Err(TxError::WrongToAddress(tx.to().clone()));
        }

        Ok(())
    }

    fn account_add_inner(&mut self, name: &str, code: Code) -> Address {
        let account = Account::new(name.to_string(), code);
        let address = account.get_address().clone();
        self.accounts.insert(address.clone(), account);

        address
    }

    /// Validity of Tx should be checked at caller side
    fn handle_tx(&mut self, tx: &Tx) -> Result<Bytes, StateError> {
        match tx.tx_type() {
            TxType::EoaToEoa => self.handle_tx_eoa_to_eoa(tx),
            TxType::DeployContract => self.handle_tx_deploy_contract(tx),
            TxType::CallContract => self.handle_tx_call_contract(tx),
        }
    }

    fn handle_tx_eoa_to_eoa(&mut self, tx: &Tx) -> Result<Bytes, StateError> {
        let from = self.accounts.get_mut(tx.from()).unwrap();

        match from.sub_balance(tx.value()) {
            Ok(_) => {
                let to = self.accounts.get_mut(tx.to()).unwrap();
                to.add_balance(tx.value());
                Ok(Bytes::new())
            }
            Err(_) => Err(StateError::TxError(TxError::NotEnoughBalance)),
        }
    }

    fn handle_tx_deploy_contract(&mut self, tx: &Tx) -> Result<Bytes, StateError> {
        let address = self.account_add_inner(tx.contract_name().unwrap(), tx.data().clone());
        let mut vm = VM::new(self.accounts.get(&address).unwrap().get_code().clone());
        let mut ext = Ext::new(address, &mut self.accounts, tx);

        match vm.execute(&mut ext) {
            Ok(vm_result) => match vm_result {
                VMResult::Ok | VMResult::Stop => Ok(Bytes::new()),
                VMResult::Return(bytes) => {
                    let account = self.accounts.get_mut(&address).unwrap();
                    account.set_code(bytes);
                    Ok(Bytes::new())
                }
            },
            Err(err) => Err(StateError::VMError(err)),
        }
    }

    fn handle_tx_call_contract(&mut self, tx: &Tx) -> Result<Bytes, StateError> {
        let account = self.accounts.get(tx.to()).unwrap();
        if !account.is_contract() {
            return Err(StateError::TxError(TxError::CallEoAAccount));
        }

        let mut vm = VM::new(account.get_code().clone());
        let mut ext = Ext::new(account.get_address().clone(), &mut self.accounts, tx);

        match vm.execute(&mut ext) {
            Ok(vm_result) => match vm_result {
                VMResult::Ok | VMResult::Stop => Ok(Bytes::new()),
                VMResult::Return(bytes) => Ok(bytes),
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
