mod account;
mod block;
mod eth_types;
mod evm;
mod hash;
mod state;
mod tx;

use std::path::PathBuf;

use self::{
    eth_types::{Address, EthFrom},
    state::{State, StateError},
};
use crate::eth_api::{AccountInfo, EthApi, EthError};

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
    fn account_add(&mut self, name: &str) {
        self.state.account_add(name);
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

    fn tx_send(&mut self, params_file: PathBuf) -> Result<(), EthError> {
        match self.state.tx_send(params_file) {
            Ok(_) => Ok(()),
            Err(err) => match err {
                StateError::NotExistedAddress(address) => {
                    #[cfg(debug_assertions)]
                    println!("{} not existed", address);

                    Err(EthError::NotExistedAddress)
                }
                StateError::NotEnoughBalance => {
                    #[cfg(debug_assertions)]
                    println!("not enough balance");

                    Err(EthError::NotEnoughBalance)
                }
                StateError::VMError(vm_error) => {
                    #[cfg(debug_assertions)]
                    println!("vm error: {:#?}", vm_error);

                    Err(EthError::VMError)
                }
            },
        }
    }
}
