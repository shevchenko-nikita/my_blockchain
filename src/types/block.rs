use blake2::{Blake2s, Digest};
use blake2::digest::FixedOutput;
use crate::types::{Hash, Transaction};

#[derive(Default, Debug)]
pub struct Block {
    nonce: u128,
    hash: Hash,
    prev_hash: Option<Hash>,
    transactions: Vec<Transaction>
}

impl Block {
    pub fn new(prev_hash: Option<Hash>) -> Self {
        Block{
            prev_hash,
            ..Default::default()
        }
    }

    pub fn set_nonce(&mut self, nonce: u128) {
        self.nonce = nonce;
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn hash(&self) -> Hash {
        let mut hasher = Blake2s::new();
        hasher.update(format!("{:?}", (self.prev_hash.clone(), self.nonce)).as_bytes());
        for tx in self.transactions.iter() {
            hasher.update(tx.hash());
        }
        hex::encode(hasher.finalize_fixed())
    }
}



#[cfg(test)]
mod tests {
    use crate::types::TransactionData;
    use super::*;

    #[test]
    fn test_creation() {
        let mut block = Block::new(None);
        let mut tx = Transaction::new(TransactionData::CreateAccount("alice".to_string()), None);
        block.set_nonce(1);
        block.add_transaction(tx);
        dbg!(block);
    }
    
    #[test]
    fn test_hash() {
        let mut block = Block::new(None);
        block.set_nonce(1);
        let hash1 = block.hash();

        let mut tx = Transaction::new(TransactionData::CreateAccount("alice".to_string()), None);
        block.transactions.push(tx);
        let hash2 = block.hash();

        assert_ne!(hash1, hash2);
    }
}