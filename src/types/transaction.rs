use crate::types::{AccountId, Balance, Timestamp};

#[derive(Debug)]
pub struct Transaction {
    nonce: u128,
    timestamp: Timestamp,
    from: Option<AccountId>,
    data: TransactionData,
    signature: Option<String>
}

impl Transaction {
    pub fn new(data : TransactionData, from : Option<AccountId>) -> Self {
        Transaction{
            nonce: 0,
            timestamp: 0,
            data,
            from,
            signature: None

        }
    }
}

#[derive(Debug)]
pub enum TransactionData {
    CreateAccount(AccountId),
    MintInitialSupply{to:AccountId, amount: Balance},
    Transfer{to: AccountId, amount: Balance}
}