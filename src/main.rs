use clap::{Parser, Subcommand};
use ethereum_simulator::{Client, Rpc, REPL};

fn main() {
    let cli = Cli::parse();

    let mut client: Box<dyn Client> = match &cli.command {
        Commands::REPL => Box::new(REPL::new()),
        Commands::Rpc { socket } => Box::new(Rpc::new(&socket)),
    };
    client.run();
}

#[derive(Parser)]
#[clap(author, version)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    REPL,
    Rpc {
        #[clap(value_parser)]
        socket: String,
    },
}
