pub enum CmdErrCode {
    INVALIDCMD,
    CMDACCOUNTERR(CmdAccountErrCode),
}

pub enum CmdAccountErrCode {
    NOTEXISTEDACCOUNT,
}
