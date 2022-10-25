use std::env::current_dir;

use block_chain::{Blockchain, SledDb};

fn main() {
    tracing_subscriber::fmt().init();

    let path = current_dir().unwrap().join("data");
    let mut bc = Blockchain::new(SledDb::new(path));

    bc.mine_block("Justin -> Bob 2 btc");
    bc.blocks_info();
}
