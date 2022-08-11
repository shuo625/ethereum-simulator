mod cmd;
pub mod cmd_errors;

use std::io::{self, Write};

use crate::eth_simulator::EthSimulator;

pub struct Cli {
    eth_simulator: EthSimulator,
}

impl Cli {
    pub fn new() -> Self {
        Cli {
            eth_simulator: EthSimulator::new(),
        }
    }

    pub fn run(&mut self) {}
}

pub fn cli_run() {
    let mut state = State::new();
    loop {
        print!(">");
        io::stdout().flush().expect("flush failed");

        let mut cmd_str = String::new();

        io::stdin()
            .read_line(&mut cmd_str)
            .expect("failed to read command");

        cmd::cmd_exec(&cmd_str, &mut state);
    }
}
