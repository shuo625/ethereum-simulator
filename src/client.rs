mod repl;
mod rpc;

pub enum Client {
    REPL(repl::REPL),
    Rpc(rpc::Rpc),
}

impl Client {
    pub fn new(arg: &str) -> Option<Self> {
        match arg {
            "REPL" => Some(Self::REPL(repl::REPL::new())),
            "Rpc" => Some(Self::Rpc(rpc::Rpc::new())),
            _ => None,
        }
    }

    pub fn run(self) {
        match self {
            Self::REPL(repl) => repl.run(),
            Self::Rpc(rpc) => rpc.run(),
        }
    }
}
