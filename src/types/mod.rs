mod transaction;
mod account;
mod blockchain;
mod block;
mod chain;

pub use chain::Chain;
pub use account::{Account, AccountType};
pub use transaction::{Transaction, TransactionData};
pub use blockchain::Blockchain;
pub use block::Block;

pub type Hash = String;
pub type Timestamp = u128;
pub type AccountId = String;
pub type Balance = u128;
pub type Error = String;