use crate::block::Block; // Block 모듈을 사용
use serde::{Deserialize, Serialize}; // Serialize와 Deserialize를 위한 Serde 크레이트

// Blockchain 구조체 정의
#[derive(Debug, Clone, Serialize, Deserialize)] // Debug, Clone, Serialize, Deserialize 트레이트를 구현
pub struct Blockchain {
    pub chain: Vec<Block>, // 블록체인에 포함될 블록들의 벡터
    pub difficulty: usize, // 블록 채굴 난이도
}

impl Blockchain {
    // 새로운 블록체인 인스턴스를 생성하는 함수
    // 초기에는 genesis 블록만 포함되며, 난이도는 기본적으로 4로 설정
    pub fn new() -> Self {
        Blockchain {
            chain: vec![Block::genesis()], // genesis 블록을 첫 번째 블록으로 추가
            difficulty: 4, // 기본 난이도 설정
        }
    }

    // 새로운 블록을 블록체인에 추가하는 함수
    pub fn add_block(&mut self, transactions: Vec<String>) {
        // 체인에서 가장 마지막 블록을 이전 블록으로 설정
        let previous_block = self.chain.last().unwrap().clone();
        
        // 새로운 블록 생성
        let mut new_block = Block::new(
            self.chain.len() as u32, // 블록 인덱스 (현재 블록체인 길이)
            current_timestamp(), // 현재 타임스탬프 (시간)
            previous_block.hash, // 이전 블록의 해시값
            transactions, // 새로운 블록에 포함할 거래 내역
        );

        // 블록 채굴을 수행하여 유효한 해시를 찾는다
        new_block.mine_block(self.difficulty);

        // 새로운 블록을 블록체인에 추가
        self.chain.push(new_block);
    }
}

// 현재 시스템의 타임스탬프를 밀리초 단위로 반환하는 함수
fn current_timestamp() -> u128 {
    std::time::SystemTime::now() // 현재 시스템 시간을 가져옴
        .duration_since(std::time::UNIX_EPOCH) // Unix epoch(1970년 1월 1일) 이후 경과된 시간 계산
        .unwrap() // unwrap()을 사용해 오류를 처리 (오류가 발생하면 패닉)
        .as_millis() // 밀리초 단위로 반환
}
