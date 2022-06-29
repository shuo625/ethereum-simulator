mod cmd;

use std::io;

use cmd::CmdExitCode;

pub fn cli_run() {
    loop {
        print!(">");

        let mut cmd = String::new();

        io::stdin()
            .read_line(&mut cmd)
            .expect("failed to read command");

        let cmd_exit_code = cmd::cmd_run(&cmd);

        match cmd_exit_code {
            CmdExitCode::SUCC => {}
            CmdExitCode::ERR => {
                println!("Wrong command");
            }
        }
    }
}
