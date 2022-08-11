mod cli;
mod rpc;

pub struct AccountInfo {
    name: String,
    address: String,
    balance: usize,
}
pub trait EthApi {
    fn account_add(&mut self, name: &str);
    fn account_list(&self) -> Vec<AccountInfo>;
    fn account_balance(&self) -> usize;

    fn tx_send(&mut self, params_file: &str);
}
