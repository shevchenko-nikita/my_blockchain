use std::collections::HashMap;
use crate::types::{Account, AccountId, Block, Transaction};

pub struct Blockchain {
    blocks: Vec<Block>,
    accounts: HashMap<AccountId, Account>,
    transaction_pool: Vec<Transaction>
}
