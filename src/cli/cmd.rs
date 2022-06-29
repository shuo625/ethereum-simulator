pub enum CmdExitCode {
    SUCC,
    ERR,
}

pub fn cmd_run(_cmd: &String) -> CmdExitCode {
    CmdExitCode::SUCC
}
