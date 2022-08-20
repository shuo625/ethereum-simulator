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
    eth_api::{AccountInfo, EthApi, EthError},
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
    fn account_add(&mut self, name: &str) -> String {
        let address = self.state.account_add(name);
        String::ethfrom(&address)
    }

    fn account_list(&self) -> Vec<AccountInfo> {
        self.state
            .account_list()
            .iter()
            .map(|(name, address, balance)| AccountInfo {
                name: name.to_string(),
                address: String::ethfrom(*address),
                balance: *balance,
            })
            .collect()
    }

    fn account_balance(&self, address: &str) -> Result<usize, EthError> {
        match self.state.account_get_balance(&Address::ethfrom(address)) {
            Ok(value) => Ok(value),
            Err(StateError::NotExistedAddress(addr)) => {
                #[cfg(debug_assertions)]
                println!("{} not existed", addr);

                Err(EthError::NotExistedAddress)
            }
            Err(_) => Err(EthError::NotExistedAddress),
        }
    }

    fn tx_send(
        &mut self,
        from: &str,
        to: &str,
        value: usize,
        data: &str,
    ) -> Result<Option<usize>, EthError> {
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
                Some(value) => Ok(Some(U256::ethfrom(value.as_slice()).as_usize())),
                None => Ok(None),
            },
            Err(err) => match err {
                StateError::NotExistedAddress(_address) => Err(EthError::NotExistedAddress),
                StateError::NotEnoughBalance => Err(EthError::NotEnoughBalance),
                #[allow(unused_variables)]
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

    fn deploy_contract(
        &mut self,
        from: &str,
        contract_file: &Path,
    ) -> Result<Option<usize>, EthError> {
        if let Ok(result) = solc::compile(contract_file) {
            self.tx_send(
                from,
                Path::new(contract_file.file_name().unwrap())
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

    fn call_contract(
        &mut self,
        from: &str,
        contract: &str,
        input: &str,
    ) -> Result<Option<usize>, EthError> {
        if let Some(to) = self.state.account_query_address_by_name(contract) {
            self.tx_send(from, String::ethfrom(&to).as_str(), 20, input)
        } else {
            Err(EthError::NotExistedContract)
        }
    }
}
