use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

// Block 구조체 정의
#[derive(Debug)]
pub struct Block {
    pub timestamp: i64,
    pub data: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
    pub hash: Vec<u8>,
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
