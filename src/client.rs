mod repl;
mod rpc;

pub enum Client<'a> {
    REPL(repl::REPL<'a>),
    Rpc(rpc::Rpc),
}

impl<'a> Client<'a> {
    pub fn new(arg: &str) -> Option<Self> {
        match arg {
            "REPL" => Some(Self::REPL(repl::REPL::new())),
            "Rpc" => Some(Self::Rpc(rpc::Rpc::new("127.0.0.1:8000"))),
            _ => None,
        }
    }

    pub fn run(self) {
        match self {
            Self::REPL(mut repl) => repl.run(),
            Self::Rpc(mut rpc) => rpc.run(),
        }
    }
}
