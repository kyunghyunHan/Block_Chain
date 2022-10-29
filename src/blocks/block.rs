use chrono::Utc;
//시간 라이브러리
//UTC:UTC 시간대를 지정,가장 효율적
use serde::{Deserialize, Serialize};
//일반 직렬화/역직렬화 프레임워크
use crate::{
    utils::{hash_to_str, serialize},
    ProofOfWork, Transaction,
};

/*
블록헤더
타임스탬프:블록이 생성된 대략적인 시간,타임스탬프에 의존하고 현재 평균 블록 생성 비율이 목표 값과 얼마나 잘 일치하는지 결정하는 스마트 계약에서 사용
prev_hash:이전시간
nonce:작업증명을 계산하는데 사용하는 난수
txs_hash:트랜잭션 집합의 해시 값, Merkle 트리로 최적화
bits:난이도,즉 블록 해시 값의 첫번쨰 비트가 0인수를 계산
nonce:비트의 난이도를 충족시키기 위해 계산이 반복되는 횟수
 */

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct BlockHeader {
    timestamp: i64,
    prev_hash: String,
    txs_hash: String,
    bits: usize,
    nonce: usize,
}

impl BlockHeader {
    fn new(prev_hash: &str, bits: usize) -> Self {
        Self {
            //초단위
            timestamp: Utc::now().timestamp(),
            //입력 값을 사용하는 값 ​​대 값 변환
            //from은 받는쪽에서 부르는거고, into는 보내는쪽에서 부르는것
            prev_hash: prev_hash.into(),
            txs_hash: String::new(),
            bits,
            nonce: 0,
        }
    }
}
/*
블록
- 헤더 :블록헤더
- tranxs : 트랜잭션 집합
- hash:블록의 해시값
각 블록의 해시값을 계산하여 저장하고 이전 블록의 해시를 블록 헤더에 저장하여 블록체인을 형성

*/
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Block {
    header: BlockHeader,
    tranxs: Vec<Transaction>,
    hash: String,
}

impl Block {
    pub fn new(txs: &[Transaction], prev_hash: &str, bits: usize) -> Self {
        let mut block = Block {
            //새로운 블록헤더 생성
            header: BlockHeader::new(prev_hash, bits),
            //새로운 Vec에 복사
            tranxs: txs.to_vec(),
            //
            hash: String::new(),
        };
        //블록집합들 해시
        block.set_txs_hash(txs);
        //pow
        let pow = ProofOfWork::new(bits);
        pow.run(&mut block);

        block
    }
    //제네시스 블록 구현
    //첫번쨰 블록이므로 블록헤더의 prev_hash에는 값이 없다.
    //제네시스 블록 생성 시 코인베이스 트랜잭션 생성
    //혹은 generation transaction
    //코인베이스 트랜잭션(coinbase transaction)은 새 블록의 첫 거래이고, 코인베이스(coinbase)는 이 거래의 '입력' 의 내용물
    //base기반
    pub fn create_genesis_block(bits: usize, genesis_addr: &str) -> Self {
        let coinbase = Transaction::new_coinbase(genesis_addr);
        Self::new(&vec![coinbase], "", bits)
    }
    //해시가져오기
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }
    //블록헤더 가져오기
    pub fn get_header(&self) -> BlockHeader {
        self.header.clone()
    }
    //nonce저장
    pub fn set_nonce(&mut self, nonce: usize) {
        self.header.nonce = nonce;
    }
    //해시저장
    pub fn set_hash(&mut self, hash: String) {
        self.hash = hash;
    }
    //트랜잭션 머클트리
    //트랜잭션들 해시
    fn set_txs_hash(&mut self, txs: &[Transaction]) {
        if let Ok(txs_ser) = serialize(txs) {
            self.header.txs_hash = hash_to_str(&txs_ser);
        }
    }
    //트랸쟉션들 자겨오기
    pub fn get_tranxs(&self) -> Vec<Transaction> {
        self.tranxs.clone()
    }
}
