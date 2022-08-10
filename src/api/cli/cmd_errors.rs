pub enum CmdErrCode {
    INVALIDCMD,
    CMDACCOUNTERR(CmdAccountErrCode),
    CMDTXERR(CmdTxErrCode),
}

pub enum CmdAccountErrCode {
    NOTEXISTEDACCOUNT,
}

pub enum CmdTxErrCode {
    NOTEXISTEDFROM,
    NOTEXISTEDTO,
    NOTENOUGHBALANCE,
}
