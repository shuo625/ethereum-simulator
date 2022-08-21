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
    state::{State, StateError},
    tx::Tx,
};
use crate::{
    eth_api::{AccountInfo, EthApi, EthError, EthResult},
    utils::solc,
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
        match self.state.account_get_balance(&Address::ethfrom(address)) {
            Ok(value) => Ok(EthResult::Value(value)),
            Err(_) => Err(EthError::NotExistedAddress),
        }
    }

    fn tx_send(
        &mut self,
        from: &str,
        to: &str,
        value: usize,
        data: &str,
    ) -> Result<EthResult, EthError> {
        let tx = if to.starts_with("0x") {
            Tx::new(
                Address::ethfrom(from),
                Address::ethfrom(to),
                String::from(""),
                value,
                Bytes::ethfrom(data),
            )
        } else {
            Tx::new(
                Address::ethfrom(from),
                Address::zero(),
                String::from(to),
                value,
                Bytes::ethfrom(data),
            )
        };

        match self.state.tx_send(tx) {
            Ok(result) => match result {
                Some(value) => Ok(EthResult::Value(U256::ethfrom(value.as_slice()).as_usize())),
                None => Ok(EthResult::None),
            },
            Err(err) => match err {
                StateError::NotExistedAddress(_address) => Err(EthError::NotExistedAddress),
                StateError::NotEnoughBalance => Err(EthError::NotEnoughBalance),
                StateError::VMError(vm_error) => {
                    #[cfg(debug_print)]
                    println!("vm error: {:#?}", vm_error);

                    Err(EthError::VMError)
                }
                StateError::CallEoAAccount => {
                    #[cfg(debug_print)]
                    println!("to address is not contract");

                    Err(EthError::CallEoAAccount)
                }
            },
        }
    }

    fn contract_deploy(&mut self, from: &str, contract_file: &str) -> Result<EthResult, EthError> {
        let file = Path::new(contract_file);
        if let Ok(result) = solc::compile(file) {
            self.tx_send(
                from,
                Path::new(file.file_name().unwrap())
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap(),
                0,
                &result,
            )
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
        if let Some(to) = self.state.account_query_address_by_name(contract) {
            self.tx_send(from, String::ethfrom(&to).as_str(), 20, input)
        } else {
            Err(EthError::NotExistedContract)
        }
    }
}
