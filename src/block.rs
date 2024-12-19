use ring::digest::{Context, Digest, SHA256}; // SHA256 해시를 생성하기 위한 ring 크레이트
use serde::{Serialize, Deserialize}; // Serde를 사용하여 직렬화 및 역직렬화

// Block 구조체 정의
#[derive(Debug, Clone, Serialize, Deserialize)] // Debug, Clone, Serialize, Deserialize 트레이트를 구현
pub struct Block {
    pub index: u32, // 블록의 인덱스 (블록체인에서의 위치)
    pub timestamp: u128, // 블록이 생성된 시간 (타임스탬프)
    pub previous_hash: String, // 이전 블록의 해시값
    pub hash: String, // 현재 블록의 해시값
    pub nonce: u64, // 채굴을 위한 nonce 값
    pub transactions: Vec<String>, // 블록에 포함된 거래 리스트
}

impl Block {
    // 새 블록을 생성하는 함수
    pub fn new(index: u32, timestamp: u128, previous_hash: String, transactions: Vec<String>) -> Self {
        let nonce = 0; // 채굴을 시작하기 위한 초기 nonce 값 (0으로 시작)
        let hash = Block::calculate_hash(index, timestamp, &previous_hash, &transactions, nonce); // 초기 해시 계산
        Block {
            index,
            timestamp,
            previous_hash,
            hash,
            nonce,
            transactions,
        }
    }

    // 주어진 인덱스, 타임스탬프, 이전 블록 해시, 거래 내역, nonce 값을 기반으로 해시를 계산하는 함수
    pub fn calculate_hash(index: u32, timestamp: u128, previous_hash: &str, transactions: &[String], nonce: u64) -> String {
        let mut context = Context::new(&SHA256); // SHA256 해시를 위한 Context 생성
        context.update(&index.to_be_bytes()); // 인덱스를 바이트로 변환하여 추가
        context.update(&timestamp.to_be_bytes()); // 타임스탬프를 바이트로 변환하여 추가
        context.update(previous_hash.as_bytes()); // 이전 블록 해시를 바이트로 추가
        context.update(&transactions.concat().as_bytes()); // 거래 리스트를 이어붙여 바이트로 추가
        context.update(&nonce.to_be_bytes()); // nonce 값을 바이트로 변환하여 추가
        let digest = context.finish(); // 해시 계산 완료
        hex::encode(digest.as_ref()) // 결과를 16진수로 인코딩하여 반환
    }

    // 주어진 난이도(difficulty)에 맞게 블록을 채굴하는 함수
    pub fn mine_block(&mut self, difficulty: usize) {
        // 목표는 난이도에 맞는 해시 앞부분이 '0'인 문자열을 생성하는 것
        let target = std::iter::repeat('0').take(difficulty).collect::<String>();

        // 목표 해시값을 찾을 때까지 nonce 값을 증가시키며 반복
        while &self.hash[..difficulty] != target {
            self.nonce += 1; // nonce 값을 증가
            // 새로운 해시값을 계산
            self.hash = Block::calculate_hash(self.index, self.timestamp, &self.previous_hash, &self.transactions, self.nonce);
            
            // 10,000번마다 진행상황을 출력 (디버깅용)
            if self.nonce % 10000 == 0 {
                println!("Nonce: {}, Hash: {}", self.nonce, self.hash);
            }
        }

        // 채굴 완료된 블록의 해시 출력
        println!("Mined block with hash: {}", self.hash);
    }

    // 제네시스 블록(블록체인의 첫 번째 블록)을 생성하는 함수
    pub fn genesis() -> Self {
        Block::new(0, 0, "0".to_string(), vec!["Genesis Block".to_string()]) // 인덱스 0, 타임스탬프 0, 이전 해시 '0', "Genesis Block" 거래
    }
}
