use std::{env::current_dir, sync::Arc};

use block_chain::{Blockchain, SledDb, UTXOSet, Wallets};

fn main() {
    tracing_subscriber::fmt().init();

    let mut wallets = Wallets::new().unwrap();
    let genesis_addr = wallets.create_wallet();
    println!("==> genesis address: {}", genesis_addr);

    let path = current_dir().unwrap().join("data");
    let storage = Arc::new(SledDb::new(path));

    let mut bc = Blockchain::new(storage.clone());
    bc.create_genesis_block(&genesis_addr);

    let utxos = UTXOSet::new(storage);
    utxos.reindex(&bc).unwrap();
    //블록 가입
    bc.blocks_info();
}
