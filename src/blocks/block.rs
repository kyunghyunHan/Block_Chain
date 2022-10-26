use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::ProofOfWork;
//블록 헤더에 비트 및  논스 필드 추가
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct BlockHeader {
    timestamp: i64,
    prev_hash: String,
    bits: usize,
    nonce: usize,
}
//블록헤더
//타임스탬프:시간저장
//prev_hash이전해시값 저장
//nonce : 저장
//bit:난이도,해시값의 첫번쨰 비트가 0인수를 계산
//bit의 난이도를 충족시키기 위해 계산이 반복되는 횟수

impl BlockHeader {
    //새로운 블록헤더
    fn new(prev_hash: &str, bits: usize) -> Self {
        Self {
            //현제시간저장
            timestamp: Utc::now().timestamp(),
            //이전해시값
            prev_hash: prev_hash.into(),
            //
            bits,
            //난수
            nonce: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct Block {
    header: BlockHeader,
    data: String,
    hash: String,
}
//:블록헤더 저장
//트랙잭션 저장
//해시값 저장
impl Block {
    //블록생성
    pub fn new(data: &str, prev_hash: &str, bits: usize) -> Self {
        let mut block = Block {
            header: BlockHeader::new(prev_hash, bits),
            data: data.into(),
            hash: String::new(),
        };
        let pow = ProofOfWork::new(bits);
        pow.run(&mut block);

        block
    }
    //제네시스 블록 생성
    pub fn create_genesis_block(bits: usize) -> Self {
        Self::new("제네시스 블록", "", bits)
    }
    //해시블러오기
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }
    //헤더 불러오기
    pub fn get_header(&self) -> BlockHeader {
        self.header.clone()
    }
    //난수 저장
    pub fn set_nonce(&mut self, nonce: usize) {
        self.header.nonce = nonce;
    }
    //해시저장
    pub fn set_hash(&mut self, hash: String) {
        self.hash = hash;
    }
}
