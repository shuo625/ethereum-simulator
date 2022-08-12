use clap::{Parser, ValueEnum};
use ethereum_simulator::Client;

fn main() {
    let args = Args::parse();

    let client = Client::new(&args.mode.to_string()).unwrap();
    client.run();
}

#[derive(Parser)]
#[clap(version)]
struct Args {
    #[clap(short, long, arg_enum, value_parser)]
    mode: Mode,
}

#[derive(ValueEnum, Debug, Clone)]
enum Mode {
    REPL,
    Rpc,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
