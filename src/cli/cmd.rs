use super::cmd_errors::{CmdAccountErrCode, CmdErrCode};
use crate::state::State;

const USAGE: &'static str = r#"
Usage:

help show help info

account add <name>
account list

tx send --from <name> --to <name> -v <value>
"#;

pub fn cmd_exec(cmd_str: &str, state: &mut State) {
    match cmd_run(cmd_str, state) {
        Err(err) => match err {
            CmdErrCode::INVALIDCMD => println!("invalid command"),
            CmdErrCode::CMDACCOUNTERR(account_err) => match account_err {
                CmdAccountErrCode::NOTEXISTEDACCOUNT => println!("account not existed"),
            },
        },
        Ok(()) => {}
    }
}

/// cmd will be the format of cmd subcmd args
pub fn cmd_run(cmd_str: &str, state: &mut State) -> Result<(), CmdErrCode> {
    let cmd_str_trim = cmd_str.trim();
    let (cmd, subcmd_str) = cmd_str_trim.split_once(' ').unwrap_or((cmd_str_trim, ""));

    match cmd {
        "help" => cmd_help(),
        "account" => cmd_account(subcmd_str, state),
        "tx" => cmd_tx(subcmd_str, state),
        _ => Err(CmdErrCode::INVALIDCMD),
    }
}

fn cmd_help() -> Result<(), CmdErrCode> {
    println!("{USAGE}");

    Ok(())
}

fn cmd_account(subcmd_str: &str, state: &mut State) -> Result<(), CmdErrCode> {
    let (subcmd, args) = subcmd_str.split_once(' ').unwrap_or((subcmd_str, ""));

    match subcmd {
        "add" => cmd_account_add(args, state),
        "list" => cmd_account_list(state),
        "show" => cmd_account_show(args, state),
        _ => Err(CmdErrCode::INVALIDCMD),
    }
}

fn cmd_account_add(args: &str, state: &mut State) -> Result<(), CmdErrCode> {
    state.account_add(args);

    Ok(())
}

fn cmd_account_list(state: &mut State) -> Result<(), CmdErrCode> {
    println!("{:?}", state.account_list());

    Ok(())
}

fn cmd_account_show(args: &str, state: &mut State) -> Result<(), CmdErrCode> {
    let name = args.trim_end();
    let balance = match state.account_get_balance(name) {
        Some(balance) => balance,
        None => {
            return Err(CmdErrCode::CMDACCOUNTERR(
                CmdAccountErrCode::NOTEXISTEDACCOUNT,
            ))
        }
    };

    println!("account: {}, balance: {}", name, balance);

    Ok(())
}

fn cmd_tx(subcmd_str: &str, state: &mut State) -> Result<(), CmdErrCode> {
    let (subcmd, args) = subcmd_str.split_once(' ').unwrap_or((subcmd_str, ""));

    match subcmd {
        "send" => cmd_tx_send(args, state),
        _ => Err(CmdErrCode::INVALIDCMD),
    }
}

fn cmd_tx_send(args: &str, state: &mut State) -> Result<(), CmdErrCode> {
    state.tx_send(args);

    Ok(())
}
