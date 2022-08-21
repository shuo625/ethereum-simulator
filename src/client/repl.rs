use easy_repl::{command, CommandStatus, Repl};
use serde::Deserialize;
use serde_json;

use std::{fs::File, io::BufReader, rc::Rc, sync::Mutex};

use super::Client;
use crate::{
    eth_api::{EthApi, EthError, EthResult},
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
                (name: String) => |name| {
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
                (address: String) => |address| {
                    let mut eth_simulator = eth_simulator_clone.lock().unwrap();
                    Self::account_balance(&*eth_simulator, address);
                    Ok(CommandStatus::Done)
                }
            },
        );

        eth_simulator_clone = eth_simulator.clone();
        repl = repl.add(
            "tx_send_file",
            command! {
                "send transaction by params file",
                (params_file: String) => |params_file| {
                    let mut eth_simulator = eth_simulator_clone.lock().unwrap();
                    Self::tx_send_file(&mut *eth_simulator, params_file);
                    Ok(CommandStatus::Done)
                }
            },
        );

        eth_simulator_clone = eth_simulator.clone();
        repl = repl.add(
            "tx_send",
            command! {
                "send transaction",
                (from: String, to: String, value: String, data: String) => |from, to, value, data| {
                    let mut eth_simulator = eth_simulator_clone.lock().unwrap();
                    Self::tx_send_file(&mut *eth_simulator, from, to, value, data);
                    Ok(CommandStatus::Done)
                }
            },
        );

        eth_simulator_clone = eth_simulator.clone();
        repl = repl.add(
            "contract_deploy",
            command! {
                "deploy contract",
                (from: String, contract_file: String) => |from, contract_file| {
                    let mut eth_simulator = eth_simulator_clone.lock().unwrap();
                    Self::contract_deploy(&mut *eth_simulator, from, contract_file);
                    Ok(CommandStatus::Done)
                }
            },
        );

        eth_simulator_clone = eth_simulator.clone();
        repl = repl.add(
            "contract_call",
            command! {
                "call contract",
                (from: String, contract: String, input: String) => |from, contract, input| {
                    let mut eth_simulator = eth_simulator_clone.lock().unwrap();
                    Self::contract_call(&mut *eth_simulator, from, contract, input);
                    Ok(CommandStatus::Done)
                }
            },
        );

        REPL {
            repl: repl.build().expect("Failed to create repl"),
        }
    }

    fn account_add(eth_simulator: &mut EthSimulator, name: String) {
        Self::handle_eth_result(eth_simulator.account_add(&name));
    }

    fn account_list(eth_simulator: &EthSimulator) {
        Self::handle_eth_result(eth_simulator.account_list());
    }

    fn account_balance(eth_simulator: &EthSimulator, address: String) {
        Self::handle_eth_result(eth_simulator.account_balance(&address));
    }

    fn tx_send_file(eth_simulator: &mut EthSimulator, params_file: String) {
        if let Ok(file) = File::open(params_file) {
            if let Ok(tx) = serde_json::from_reader::<BufReader<File>, Tx>(BufReader::new(file)) {
                Self::handle_eth_result(eth_simulator.tx_send(
                    &tx.from,
                    &tx.to,
                    tx.value.parse::<usize>().unwrap(),
                    &tx.data,
                ));
            } else {
                println!("wrong file format, failed to deserialize file")
            }
        } else {
            println!("failed to open the file, check the path of file")
        }
    }

    fn tx_send(
        eth_simulator: &mut EthSimulator,
        from: String,
        to: String,
        value: String,
        data: String,
    ) {
        Self::handle_eth_result(eth_simulator.tx_send(
            &from,
            &to,
            value.parse::<usize>().unwrap(),
            &data,
        ));
    }

    fn contract_deploy(eth_simulator: &mut EthSimulator, from: String, contract_file: String) {
        Self::handle_eth_result(eth_simulator.contract_deploy(&from, &contract_file));
    }

    fn contract_call(
        eth_simulator: &mut EthSimulator,
        from: String,
        contract: String,
        input: String,
    ) {
        Self::handle_eth_result(eth_simulator.contract_call(&from, &contract, &input));
    }

    fn handle_eth_result(result: Result<EthResult, EthError>) {
        match result {
            Ok(value) => match value {
                EthResult::AccountList(accounts) => {
                    for account in accounts {
                        println!(
                            "name: {}, address: {}, balance: {}",
                            account.name, account.address, account.balance
                        );
                    }
                }
                EthResult::Address(address) => println!("address: {}", address),
                EthResult::Value(value) => println!("value: {}", value),
                EthResult::None => {}
            },
            Err(err) => match err {
                EthError::NotExistedAddress => println!("some address does not exist"),
                EthError::NotEnoughBalance => println!("balance is not enough"),
                EthError::VMError => println!("there is a vm error"),
                EthError::CallEoAAccount => println!("called account is not Contract"),
                EthError::NotExistedContract => println!("called contract does not exist"),
                EthError::CompileError => {
                    println!("compiling contract failed, check code or path of contract")
                }
            },
        }
    }
}

/// Tx struct for serde json deserialize
#[derive(Deserialize, Debug)]
struct Tx {
    from: String,
    to: String,
    value: String,
    data: String,
}
