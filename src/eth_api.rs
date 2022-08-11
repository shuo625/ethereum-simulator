pub struct AccountInfo {
    pub name: String,
    pub address: String,
    pub balance: usize,
}

pub enum EthError {
    NotExistedAddress,
    NotEnoughBalance,
    VMError,
}
pub trait EthApi {
    fn account_add(&mut self, name: &str);
    fn account_list(&self) -> Vec<AccountInfo>;
    fn account_balance(&self, address: &str) -> Result<usize, EthError>;

    fn tx_send(&mut self, params_file: &str) -> Result<(), EthError>;
}
