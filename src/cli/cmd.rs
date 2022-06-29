pub enum CmdExitCode {
    SUCC,
    ERR,
}

pub fn cmd_run(cmd: &String) -> CmdExitCode {
    println!("Execute command {}", cmd);
    CmdExitCode::SUCC
}
