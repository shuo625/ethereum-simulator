mod cmd;

use std::io::{self, Write};

use cmd::CmdExitCode;

pub fn cli_run() {
    loop {
        print!(">");
        io::stdout().flush().expect("flush failed");

        let mut cmd = String::new();

        io::stdin()
            .read_line(&mut cmd)
            .expect("failed to read command");

        let cmd_exit_code = cmd::cmd_run(&cmd);

        match cmd_exit_code {
            CmdExitCode::SUCC => {}
            CmdExitCode::ERR => {
                println!("wrong command");
            }
        }
    }
}
