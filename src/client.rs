mod cli;
mod rpc;

pub enum Client {
    Cli(cli::Cli),
    Rpc(rpc::Rpc),
}

impl Client {
    pub fn new(arg: &str) -> Option<Self> {
        match arg {
            "cli" => Some(Self::Cli(cli::Cli::new())),
            "rpc" => Some(Self::Rpc(rpc::Rpc::new())),
            _ => None,
        }
    }

    pub fn run(self) {
        match self {
            Self::Cli(cli) => cli.run(),
            Self::Rpc(rpc) => rpc.run(),
        }
    }
}
