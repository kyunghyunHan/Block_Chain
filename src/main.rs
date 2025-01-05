use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use hex;

// Block 구조체 정의
#[derive(Debug)]
pub struct Block {
    timestamp: i64,
    data: Vec<u8>,
    prev_block_hash: Vec<u8>,
    hash: Vec<u8>,
}

impl Block {
    // 해시 설정 메서드
    fn set_hash(&mut self) {
        let timestamp = self.timestamp.to_string().into_bytes();
        let headers: Vec<u8> = [&self.prev_block_hash[..], &self.data[..], &timestamp[..]].concat();
        let mut hasher = Sha256::new();
        hasher.update(headers);
        self.hash = hasher.finalize().to_vec();
    }

    // 새 블록 생성
    pub fn new(data: &str, prev_block_hash: Vec<u8>) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        let mut block = Block {
            timestamp,
            data: data.as_bytes().to_vec(),
            prev_block_hash,
            hash: vec![],
        };
        
        block.set_hash();
        block
    }

    // 제네시스 블록 생성
    pub fn new_genesis_block() -> Block {
        Block::new("Genesis Block", vec![])
    }
}

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

fn main() {
    let mut bc = Blockchain::new();
    bc.add_block("Send 1 BTC to Ivan");
    bc.add_block("Send 2 more BTC to Ivan");
    bc.print_blocks();
}