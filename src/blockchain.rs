use crate::block::Block;
use serde::{Serialize,Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { chain: vec![Block::genesis()], difficulty: 4 } // 난이도를 설정합니다.
    }

    pub fn add_block(&mut self, transactions: Vec<String>) {
        let previous_block = self.chain.last().unwrap().clone();
        let mut new_block = Block::new(
            self.chain.len() as u32,
            current_timestamp(),
            previous_block.hash,
            transactions,
        );
        new_block.mine_block(self.difficulty); // 채굴을 수행합니다.
        self.chain.push(new_block);
    }
}

fn current_timestamp() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
