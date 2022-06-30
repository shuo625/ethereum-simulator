use crate::state::State;

pub enum CmdExitCode {
    SUCC,
    ERR,
}

const USAGE: &'static str = r#"
Usage:

help show help info

account add <name>
account list

tx send --from <name> --to <name> -v <value>
"#;

/// cmd will be the format of cmd subcmd args
pub fn cmd_run(cmd_str: String, state: &mut State) -> CmdExitCode {
    let (cmd, subcmd_str) = match cmd_str.split_once(' ') {
        Some((a, b)) => (a, b),
        None => return CmdExitCode::ERR,
    };

    match cmd {
        "help" => cmd_help(),
        "account" => cmd_account(subcmd_str.to_string(), state),
        "tx" => cmd_tx(subcmd_str.to_string(), state),
        _ => CmdExitCode::ERR,
    }
}

fn cmd_help() -> CmdExitCode {
    println!("{}", USAGE);

    CmdExitCode::SUCC
}

fn cmd_account(cmd_str: String, state: &mut State) -> CmdExitCode {
    let (cmd, args) = match cmd_str.split_once(' ') {
        Some((a, b)) => (a, b),
        None => return CmdExitCode::ERR,
    };

    match cmd {
        "add" => cmd_account_add(args.to_string(), state),
        "list" => cmd_account_list(args.to_string(), state),
        "show" => cmd_account_show(args.to_string(), state),
        _ => CmdExitCode::ERR,
    }
}

fn cmd_account_add(args: String, state: &mut State) -> CmdExitCode {
    state.account_add(args);

    CmdExitCode::SUCC
}

fn cmd_account_list(_args: String, state: &mut State) -> CmdExitCode {
    println!("accounts: {}", state.account_list());

    CmdExitCode::SUCC
}

fn cmd_account_show(args: String, state: &mut State) -> CmdExitCode {
    let name = args;
    println!(
        "account: {}, balance: {}",
        name.trim_end(),
        state.account_get_balance(&name)
    );

    CmdExitCode::SUCC
}

fn cmd_tx(cmd_str: String, state: &mut State) -> CmdExitCode {
    let (cmd, args) = match cmd_str.split_once(' ') {
        Some((a, b)) => (a, b),
        None => return CmdExitCode::ERR,
    };

    match cmd {
        "send" => cmd_tx_send(args.to_string(), state),
        _ => CmdExitCode::ERR,
    }
}

fn cmd_tx_send(args: String, state: &mut State) -> CmdExitCode {
    state.tx_send(args);

    CmdExitCode::SUCC
}
