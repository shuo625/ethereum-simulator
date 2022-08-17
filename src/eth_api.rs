use serde::Serialize;

#[derive(Serialize)]
pub struct AccountInfo {
    pub name: String,
    pub address: String,
    pub balance: usize,
}

pub enum EthError {
    NotExistedAddress,
    NotEnoughBalance,
    VMError,
    CALLEOAACCOUNT,
}
pub trait EthApi {
    fn account_add(&mut self, name: &str) -> String;
    fn account_list(&self) -> Vec<AccountInfo>;
    fn account_balance(&self, address: &str) -> Result<usize, EthError>;

    fn tx_send(&mut self, from: &str, to: &str, value: usize, data: &str) -> Result<(), EthError>;
}
