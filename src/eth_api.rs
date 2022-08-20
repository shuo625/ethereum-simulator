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
    CallEoAAccount,
    CompileError,
    NotExistedContract,
}

pub enum EthResult {
    Address(String),
    AccountList(Vec<AccountInfo>),
    Value(usize),
    None,
}

pub trait EthApi {
    fn account_add(&mut self, name: &str) -> Result<EthResult, EthError>;
    fn account_list(&self) -> Result<EthResult, EthError>;
    fn account_balance(&self, address: &str) -> Result<EthResult, EthError>;

    fn tx_send(
        &mut self,
        from: &str,
        to: &str,
        value: usize,
        data: &str,
    ) -> Result<EthResult, EthError>;

    fn deploy_contract(&mut self, from: &str, contract_file: &str) -> Result<EthResult, EthError>;

    fn call_contract(
        &mut self,
        from: &str,
        contract: &str,
        input: &str,
    ) -> Result<EthResult, EthError>;
}
