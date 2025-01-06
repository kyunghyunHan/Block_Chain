use crate::proof_of_work::ProofOfWork;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)] // Clone trait 추가
pub struct Block {
    pub timestamp: i64,
    pub data: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
    pub hash: Vec<u8>,
    pub nonce: u16, // u32에서 u16으로 변경
}

impl Block {
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
            nonce: 0,
        };

        let pow = ProofOfWork::new(&block, 8);
        let (nonce, hash) = pow.run();

        block.hash = hash;
        block.nonce = nonce;

        block
    }

    // 제네시스 블록 생성
    pub fn new_genesis_block() -> Block {
        Block::new("Genesis Block", vec![])
    }
}
