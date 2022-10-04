use crate::types::Balance;

pub enum AccountType {
    User,
    Contract,
}

pub struct Account {
    account_hide: AccountType,
    balance: Balance,
}