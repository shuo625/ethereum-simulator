mod account;
mod block;
mod eth_types;
mod evm;
mod hash;
mod state;
mod tx;

use std::path::Path;

use self::{
    eth_types::{Address, Bytes, EthFrom, U256},
    state::{State, StateError, TxError},
    tx::{Tx, TxType},
};
use crate::{
    eth_api::{AccountInfo, EthApi, EthError, EthResult},
    utils::{path, solc},
};

pub struct EthSimulator {
    state: State,
}

impl EthSimulator {
    pub fn new() -> Self {
        EthSimulator {
            state: State::new(),
        }
    }

    fn get_address(&self, addr: &str) -> Option<Address> {
        if addr.starts_with("0x") {
            let address = Address::ethfrom(addr);
            if self.state.address_exist(&address) {
                Some(address)
            } else {
                None
            }
        } else {
            self.state.account_query_address_by_name(addr)
        }
    }
}

impl EthApi for EthSimulator {
    fn account_add(&mut self, name: &str) -> Result<EthResult, EthError> {
        let address = self.state.account_add(name);
        Ok(EthResult::Address(String::ethfrom(&address)))
    }

    fn account_list(&self) -> Result<EthResult, EthError> {
        Ok(EthResult::AccountList(
            self.state
                .account_list()
                .iter()
                .map(|(name, address, balance)| AccountInfo {
                    name: name.to_string(),
                    address: String::ethfrom(*address),
                    balance: *balance,
                })
                .collect(),
        ))
    }

    fn account_balance(&self, address: &str) -> Result<EthResult, EthError> {
        Ok(EthResult::Value(
            self.state
                .account_get_balance(
                    &self
                        .get_address(address)
                        .ok_or(EthError::NotExistedAddress)?,
                )
                .unwrap(),
        ))
    }

    fn tx_send(
        &mut self,
        from: &str,
        to: &str,
        value: usize,
        data: &str,
    ) -> Result<EthResult, EthError> {
        let from_addr = self.get_address(from).ok_or(EthError::NotExistedAddress)?;
        let tx = if let Some(to_addr) = self.get_address(to) {
            let tx_type = if self.state.address_is_contract(&to_addr) {
                TxType::CallContract
            } else {
                TxType::EoaToEoa
            };

            Tx::new(
                from_addr,
                to_addr,
                value,
                Bytes::ethfrom(data),
                tx_type,
                String::new(),
            )
        } else {
            Tx::new(
                from_addr,
                Address::zero(),
                value,
                Bytes::ethfrom(data),
                TxType::DeployContract,
                to.to_string(),
            )
        };

        match self.state.tx_send(tx) {
            Ok(result) => {
                if result.len() > 0 {
                    Ok(EthResult::Value(
                        U256::ethfrom(result.as_slice()).as_usize(),
                    ))
                } else {
                    Ok(EthResult::None)
                }
            }
            Err(err) => match err {
                StateError::TxError(tx_error) => match tx_error {
                    TxError::WrongFromAddress(_) | TxError::WrongToAddress(_) => {
                        Err(EthError::NotExistedAddress)
                    }
                    TxError::NotEnoughBalance => Err(EthError::NotEnoughBalance),
                    TxError::CallEoAAccount => Err(EthError::CallEoAAccount),
                },
                StateError::VMError(_vm_error) => {
                    #[cfg(feature = "debug_print")]
                    println!("{:#?}", _vm_error);

                    Err(EthError::VMError)
                }
            },
        }
    }

    fn contract_deploy(&mut self, from: &str, contract_file: &str) -> Result<EthResult, EthError> {
        let contract = Path::new(contract_file);

        if let Ok(result) = solc::compile(contract) {
            self.tx_send(from, path::get_file_name(contract), 0, &result)
        } else {
            Err(EthError::CompileError)
        }
    }

    fn contract_call(
        &mut self,
        from: &str,
        contract: &str,
        input: &str,
    ) -> Result<EthResult, EthError> {
        if self.get_address(contract) == None {
            return Err(EthError::NotExistedContract);
        }
        self.tx_send(from, contract, 20, input)
    }
}
