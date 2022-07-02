mod cmd;
pub(self) mod cmd_errors;

use std::io::{self, Write};

use crate::state::State;

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
