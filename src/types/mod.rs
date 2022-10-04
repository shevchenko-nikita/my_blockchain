mod transaction;
mod account;
mod blockchain;
mod block;

pub use account::{Account, AccountType};
pub use transaction::{Transaction, TransactionData};
pub use blockchain::Blockchain;
pub use block::Block;

pub type Hash = String;
pub type Timestamp = u128;
pub type AccountId = String;
pub type Balance = u128;