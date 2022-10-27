use serde::{Deserialize, Serialize};

use crate::Block;
/*
제네시스: 블록체인 만들기
블록: 블록체인 정보 표시
동기화: 동기화 블록
CreateWallet: 지갑 생성
GetAddress: 주소 가져오기
Trans: 트랜잭션 생성

*/
#[derive(Debug, Serialize, Deserialize)]
pub enum Commands {
    Genesis(String),
    Blocks(String),
    Sync(String),
    CreateWallet(String),
    GetAddress(String),
    Trans {
        from: String,
        to: String,
        amount: String,
    },
}
/*

버전: 로컬 노드의 블록체인 높이를 다른 노드로 전송하여 로컬 노드를 동기화
블록: 로컬 블록체인 정보를 다른 노드로 전송
블록: 새로 추가된 블록을 다른 노드로 전송
*/
#[derive(Debug, Serialize, Deserialize)]
pub enum Messages {
    Version {
        best_height: usize,
        from_addr: String,
    },
    Blocks {
        blocks: Vec<Block>,
        height: usize,
        to_addr: String,
    },
    Block {
        block: Block,
    },
}
