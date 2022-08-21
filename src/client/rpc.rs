use serde::Deserialize;
use serde_json::{self, json, Value};

use std::{
    collections::HashMap,
    io::Write,
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use super::Client;
use crate::{
    eth_api::{EthApi, EthResult},
    eth_simulator::EthSimulator,
};

#[derive(Deserialize, Debug)]
struct Request {
    method: String,
    params: HashMap<String, String>,
}

enum RpcError {
    WrongMethod,
    WrongParams,
    WrongRequest,
}
pub struct Rpc {
    server: TcpListener,
    socket: String,
}

impl Client for Rpc {
    fn run(&mut self) {
        println!("rpc server listens at {}", self.socket);

        let eth_simulator = Arc::new(Mutex::new(EthSimulator::new()));

        for stream in self.server.incoming() {
            let stream = stream.unwrap();
            let eth_simulator_clone = eth_simulator.clone();

            println!("New connection: {}", stream.peer_addr().unwrap());

            thread::spawn(move || {
                let mut eth_simulator = eth_simulator_clone.lock().unwrap();
                Self::handle_connection(&mut eth_simulator, stream);
            });
        }
    }
}

impl Rpc {
    pub fn new(socket: &str) -> Self {
        Rpc {
            server: TcpListener::bind(socket).unwrap(),
            socket: socket.to_string(),
        }
    }

    fn handle_connection(eth_simulator: &mut EthSimulator, mut stream: TcpStream) {
        let mut status = "ok";
        let mut result = Value::Null;

        let mut de = serde_json::Deserializer::from_reader(&stream);
        if let Ok(request) = Request::deserialize(&mut de) {
            println!("receive request {:?}", request);

            match Self::handle_request(eth_simulator, request) {
                Ok(rst) => {
                    result = rst;
                }
                Err(_) => {
                    status = "error";
                }
            }
        } else {
            status = "error";
        }

        let response = json!({
            "status": status,
            "result": result
        });
        stream
            .write_all(serde_json::to_string(&response).unwrap().as_bytes())
            .unwrap();
    }

    fn handle_request(
        eth_simulator: &mut EthSimulator,
        request: Request,
    ) -> Result<Value, RpcError> {
        match request.method.as_str() {
            "account_add" => Self::account_add(eth_simulator, &request.params),
            "account_list" => Self::account_list(eth_simulator, &request.params),
            "account_balance" => Self::account_balance(eth_simulator, &request.params),
            "tx_send" => Self::tx_send(eth_simulator, &request.params),
            "contract_deploy" => Self::contract_deploy(eth_simulator, &request.params),
            "contract_call" => Self::contract_call(eth_simulator, &request.params),
            _ => Err(RpcError::WrongMethod),
        }
    }

    fn account_add(
        eth_simulator: &mut EthSimulator,
        params: &HashMap<String, String>,
    ) -> Result<Value, RpcError> {
        if let Some(name) = params.get("name") {
            if let Ok(EthResult::Address(address)) = eth_simulator.account_add(name) {
                Ok(json!({ "address": address }))
            } else {
                Err(RpcError::WrongRequest)
            }
        } else {
            Err(RpcError::WrongParams)
        }
    }

    fn account_list(
        eth_simulator: &EthSimulator,
        _params: &HashMap<String, String>,
    ) -> Result<Value, RpcError> {
        if let Ok(EthResult::AccountList(account_list)) = eth_simulator.account_list() {
            Ok(json!({ "accounts": account_list }))
        } else {
            Err(RpcError::WrongRequest)
        }
    }

    fn account_balance(
        eth_simulator: &EthSimulator,
        params: &HashMap<String, String>,
    ) -> Result<Value, RpcError> {
        if let Some(address) = params.get("address") {
            if let Ok(EthResult::Value(balance)) = eth_simulator.account_balance(address) {
                Ok(json!({ "balance": balance }))
            } else {
                Err(RpcError::WrongParams)
            }
        } else {
            Err(RpcError::WrongParams)
        }
    }

    fn tx_send(
        eth_simulator: &mut EthSimulator,
        params: &HashMap<String, String>,
    ) -> Result<Value, RpcError> {
        if let (Some(from), Some(to), Some(value), Some(data)) = (
            params.get("from"),
            params.get("to"),
            params.get("value"),
            params.get("data"),
        ) {
            match eth_simulator.tx_send(from, to, value.parse::<usize>().unwrap(), data) {
                Ok(result) => match result {
                    EthResult::Value(value) => Ok(Value::String(value.to_string())),
                    _ => Ok(Value::Null),
                },
                Err(_) => Err(RpcError::WrongRequest),
            }
        } else {
            Err(RpcError::WrongParams)
        }
    }

    fn contract_deploy(
        eth_simulator: &mut EthSimulator,
        params: &HashMap<String, String>,
    ) -> Result<Value, RpcError> {
        if let (Some(from), Some(contract_file)) = (params.get("from"), params.get("contract_file"))
        {
            match eth_simulator.contract_deploy(&from, &contract_file) {
                Ok(_) => Ok(Value::Null),
                Err(_) => Err(RpcError::WrongRequest),
            }
        } else {
            Err(RpcError::WrongParams)
        }
    }

    fn contract_call(
        eth_simulator: &mut EthSimulator,
        params: &HashMap<String, String>,
    ) -> Result<Value, RpcError> {
        if let (Some(from), Some(contract), Some(input)) = (
            params.get("from"),
            params.get("contract"),
            params.get("input"),
        ) {
            match eth_simulator.contract_call(&from, &contract, &input) {
                Ok(result) => match result {
                    EthResult::Value(value) => Ok(Value::String(value.to_string())),
                    _ => Ok(Value::Null),
                },
                Err(_) => Err(RpcError::WrongRequest),
            }
        } else {
            Err(RpcError::WrongParams)
        }
    }
}
