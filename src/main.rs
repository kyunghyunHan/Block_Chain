mod block;
mod blockchain;
use blockchain::Blockchain;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut bc = Blockchain::new();
    bc.add_block("Send 1 BTC to Ivan");
    bc.add_block("Send 2 more BTC to Ivan");
    bc.print_blocks();
}
