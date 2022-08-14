mod repl;
mod rpc;

pub use repl::REPL;
pub use rpc::Rpc;

pub trait Client {
    fn run(&mut self);
}
