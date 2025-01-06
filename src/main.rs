mod block;
mod blockchain;
mod proof_of_work;
use blockchain::Blockchain;
use proof_of_work::ProofOfWork;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut bc = Blockchain::new();
    bc.add_block("Send 1 BTC to Ivan");
    bc.add_block("Send 2 more BTC to Ivan");

    for block in bc.blocks {
        println!("Prev. hash: {:?}", hex::encode(&block.prev_block_hash));
        println!("Data: {}", String::from_utf8_lossy(&block.data));
        println!("Hash: {:?}", hex::encode(&block.hash));

        let pow = ProofOfWork::new(block); // block을 복제하여 사용
        println!("PoW: {}", pow.validate());
        println!();
    }
    // bc.print_blocks();
}
