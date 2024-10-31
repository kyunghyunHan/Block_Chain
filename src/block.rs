use ring::digest::{Context, Digest, SHA256};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub transactions: Vec<String>,
}

impl Block {
    pub fn new(index: u32, timestamp: u128, previous_hash: String, transactions: Vec<String>) -> Self {
        let nonce = 0;
        let hash = Block::calculate_hash(index, timestamp, &previous_hash, &transactions, nonce);
        Block {
            index,
            timestamp,
            previous_hash,
            hash,
            nonce,
            transactions,
        }
    }

    pub fn calculate_hash(index: u32, timestamp: u128, previous_hash: &str, transactions: &[String], nonce: u64) -> String {
        let mut context = Context::new(&SHA256);
        context.update(&index.to_be_bytes());
        context.update(&timestamp.to_be_bytes());
        context.update(previous_hash.as_bytes());
        context.update(&transactions.concat().as_bytes());
        context.update(&nonce.to_be_bytes());
        let digest = context.finish();
        hex::encode(digest.as_ref())
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = std::iter::repeat('0').take(difficulty).collect::<String>();

        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = Block::calculate_hash(self.index, self.timestamp, &self.previous_hash, &self.transactions, self.nonce);
            if self.nonce % 10000 == 0 {
                println!("Nonce: {}, Hash: {}", self.nonce, self.hash);
            }
        }

        println!("Mined block with hash: {}", self.hash);
    }

    pub fn genesis() -> Self {
        Block::new(0, 0, "0".to_string(), vec!["Genesis Block".to_string()])
    }
}
