use crate::block::Block;
// Blockchain 구조체 정의
pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    // 새 블록체인 생성
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: vec![Block::new_genesis_block()],
        }
    }

    // 블록 추가
    pub fn add_block(&mut self, data: &str) {
        let prev_block = &self.blocks[self.blocks.len() - 1];
        let new_block = Block::new(data, prev_block.hash.clone());
        self.blocks.push(new_block);
    }

    // 블록체인의 모든 블록 출력
    pub fn print_blocks(&self) {
        for block in &self.blocks {
            println!("Prev. hash: {}", hex::encode(&block.prev_block_hash));
            println!("Data: {}", String::from_utf8_lossy(&block.data));
            println!("Hash: {}", hex::encode(&block.hash));
            println!();
        }
    }
}
