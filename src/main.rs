use std::env::current_dir;

use block_chain::{Blockchain, SledDb};
//블록체인 생성 및 블록 가입
fn main() {
    tracing_subscriber::fmt().init();

    let path = current_dir().unwrap().join("data");
    //데이터베이스에 저장
    let mut bc = Blockchain::new(SledDb::new(path));

    //채굴
    bc.mine_block("Justin -> Bob 2 btc");
    //가입
    bc.blocks_info();
}
