mod account;
mod block;
mod eth_types;
mod evm;
mod hash;
mod state;
mod tx;

use self::{
    eth_types::{Address, Bytes, EthFrom, U256},
    state::{State, StateError},
    tx::Tx,
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
        match self.state.tx_send(Tx::new(
            Address::ethfrom(from),
            Address::ethfrom(to),
            value,
            Bytes::ethfrom(data),
        )) {
            Ok(result) => match result {
                Some(value) => Ok(Some(U256::ethfrom(value.as_slice()).as_usize())),
                None => Ok(None),
            },
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
                StateError::CALLEOAACCOUNT => {
                    #[cfg(debug_assertions)]
                    println!("to address is not contract");

                    Err(EthError::CALLEOAACCOUNT)
                }
            },
        }
    }
}
