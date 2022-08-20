use serde::Serialize;

use std::path::Path;

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
    CallEoAAccount,
    CompileError,
    NotExistedContract,
}
pub trait EthApi {
    fn account_add(&mut self, name: &str) -> String;
    fn account_list(&self) -> Vec<AccountInfo>;
    fn account_balance(&self, address: &str) -> Result<usize, EthError>;

    fn tx_send(
        &mut self,
        from: &str,
        to: &str,
        value: usize,
        data: &str,
    ) -> Result<Option<usize>, EthError>;

    fn deploy_contract(
        &mut self,
        from: &str,
        contract_file: &Path,
    ) -> Result<Option<usize>, EthError>;

    fn call_contract(
        &mut self,
        from: &str,
        contract: &str,
        input: &str,
    ) -> Result<Option<usize>, EthError>;
}
