use std::env::current_dir;
//	current_dir:현재 작업 디렉토리를 PathBuf.
use block_chain::{Blockchain, SledDb};
//블록체인 생성 및 블록 가입
fn main() {
    //'추적' 구독자를 구현하고 구성하기 위한 유틸리티
    tracing_subscriber::fmt().init();
    //데이터베이스에 저장
    let path = current_dir().unwrap().join("data");

    let mut bc = Blockchain::new(SledDb::new(path));
    //채굴
    bc.mine_block("Justin -> Bob 2 btc");
    //가입
    bc.blocks_info();
}
