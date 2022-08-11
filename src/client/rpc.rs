use crate::eth_simulator::EthSimulator;

pub struct Rpc {
    eth_simulator: EthSimulator,
}

impl Rpc {
    pub fn new() -> Self {
        Rpc {
            eth_simulator: EthSimulator::new(),
        }
    }
}
