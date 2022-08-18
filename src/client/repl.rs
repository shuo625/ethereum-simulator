use easy_repl::{command, CommandStatus, Repl};
use serde::Deserialize;
use serde_json;

use std::{fs::File, io::BufReader, rc::Rc, sync::Mutex};

use super::Client;
use crate::{
    eth_api::{EthApi, EthError},
    eth_simulator::EthSimulator,
};

pub struct REPL<'a> {
    repl: Repl<'a>,
}

impl Client for REPL<'_> {
    fn run(&mut self) {
        self.repl.run().expect("Critical REPL error");
    }
}

impl<'a> REPL<'a> {
    pub fn new() -> Self {
        let eth_simulator = Rc::new(Mutex::new(EthSimulator::new()));
        let mut repl = Repl::builder();

        let mut eth_simulator_clone = eth_simulator.clone();
        repl = repl.add(
            "account_add",
            command! {
                "add account",
                (name: String) => |name: String| {
                    let mut eth_simulator = eth_simulator_clone.lock().unwrap();
                    Self::account_add(&mut *eth_simulator, name);
                    Ok(CommandStatus::Done)
                }
            },
        );

        eth_simulator_clone = eth_simulator.clone();
        repl = repl.add(
            "account_list",
            command! {
                "list accounts",
                () => || {
                    let mut eth_simulator = eth_simulator_clone.lock().unwrap();
                    Self::account_list(&*eth_simulator);
                    Ok(CommandStatus::Done)
                }
            },
        );

        eth_simulator_clone = eth_simulator.clone();
        repl = repl.add(
            "account_balance",
            command! {
                "get account balance",
                (address: String) => |address: String| {
                    let mut eth_simulator = eth_simulator_clone.lock().unwrap();
                    Self::account_balance(&*eth_simulator, address);
                    Ok(CommandStatus::Done)
                }
            },
        );

        eth_simulator_clone = eth_simulator.clone();
        repl = repl.add(
            "tx_send",
            command! {
                "send transaction",
                (params_file: String) => |params_file: String| {
                    let mut eth_simulator = eth_simulator_clone.lock().unwrap();
                    Self::tx_send(&mut *eth_simulator, params_file);
                    Ok(CommandStatus::Done)
                }
            },
        );

        REPL {
            repl: repl.build().expect("Failed to create repl"),
        }
    }

    fn account_add(eth_simulator: &mut EthSimulator, name: String) {
        eth_simulator.account_add(&name);
    }

    fn account_list(eth_simulator: &EthSimulator) {
        let accounts = eth_simulator.account_list();

        for account in accounts {
            println!(
                "name: {}, address: {}, balance: {}",
                account.name, account.address, account.balance
            );
        }
    }

    fn account_balance(eth_simulator: &EthSimulator, address: String) {
        match eth_simulator.account_balance(&address) {
            Ok(balance) => println!("balance: {}", balance),
            Err(err) => match err {
                EthError::NotExistedAddress => println!("This address does not exist"),
                _ => {}
            },
        }
    }

    fn tx_send(eth_simulator: &mut EthSimulator, params_file: String) {
        if let Ok(file) = File::open(params_file) {
            if let Ok(tx) = serde_json::from_reader::<BufReader<File>, Tx>(BufReader::new(file)) {
                match eth_simulator.tx_send(&tx.from, &tx.to, tx.value, &tx.data) {
                    Ok(result) => match result {
                        Some(value) => println!("Result: {}", value),
                        None => {}
                    },
                    Err(err) => match err {
                        EthError::NotExistedAddress => println!("some address does not exist"),
                        EthError::NotEnoughBalance => println!("balance is not enough"),
                        EthError::VMError => println!("there is a vm error"),
                        EthError::CallEoAAccount => println!("called account is not Contract"),
                    },
                }
            } else {
                println!("wrong file format, failed to deserialize file")
            }
        } else {
            println!("failed to open the file, check the path of file")
        }
    }
}

/// Tx struct for serde json deserialize
#[derive(Deserialize)]
struct Tx {
    from: String,
    to: String,
    value: usize,
    data: String,
}
