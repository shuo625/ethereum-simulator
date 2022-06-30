mod cmd;

use std::io::{self, Write};

use crate::state::State;
use cmd::CmdExitCode;

pub fn cli_run() {
    let mut state = State::new();
    loop {
        print!(">");
        io::stdout().flush().expect("flush failed");

        let mut cmd = String::new();

        io::stdin()
            .read_line(&mut cmd)
            .expect("failed to read command");

        let cmd_exit_code = cmd::cmd_run(cmd, &mut state);

        match cmd_exit_code {
            CmdExitCode::SUCC => {}
            CmdExitCode::ERR => {
                println!("wrong command");
            }
        }
    }
}
