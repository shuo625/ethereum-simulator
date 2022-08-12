use std::io::{self, Write};

use crate::{
    eth_api::{AccountInfo, EthApi, EthError},
    eth_simulator::EthSimulator,
};

const USAGE: &'static str = r#"
Usage:

help show help info

account add <name>
account list

tx send --params <file>
"#;

pub struct REPL {
    eth_simulator: EthSimulator,
}

impl REPL {
    pub fn new() -> Self {
        REPL {
            eth_simulator: EthSimulator::new(),
        }
    }

    pub fn run(self) {
        loop {
            print!(">");
            io::stdout().flush().expect("flush failed");

            let mut cmd_str = String::new();

            io::stdin()
                .read_line(&mut cmd_str)
                .expect("failed to read command");
        }
    }

    fn exec_cmd(&mut self, cmd_str: &str) {}
}
