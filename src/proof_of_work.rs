use crate::block::Block;
use num_bigint::BigInt;
use sha2::{Digest, Sha256};
use std::convert::TryInto;
use std::fmt::Write;

const TARGET_BITS: i32 = 24;
const MAX_NONCE: i64 = i64::MAX;

pub struct ProofOfWork {
    block: Block,
    target: BigInt,
}

impl ProofOfWork {
    pub fn new(block: Block) -> ProofOfWork {
        let mut target = BigInt::from(1);
        target = target << (256 - TARGET_BITS);
        ProofOfWork { block, target }
    }

    fn prepare_data(&self, nonce: u16) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend(&self.block.prev_block_hash);
        data.extend(&self.block.data);
        data.extend(&self.block.timestamp.to_be_bytes());
        data.extend(&TARGET_BITS.to_be_bytes());
        data.extend(&nonce.to_be_bytes());
        data
    }

    pub fn run(&self) -> (u16, Vec<u8>) {
        let mut nonce: u16 = 0;
        let mut hash = Vec::new();

        println!(
            "Mining the block containing {:?}",
            String::from_utf8_lossy(&self.block.data)
        );

        while (nonce as i64) < MAX_NONCE {
            let data = self.prepare_data(nonce);
            let mut hasher = Sha256::new();
            hasher.update(&data);
            hash = hasher.finalize().to_vec();

            let mut hash_hex = String::new();
            for byte in &hash {
                write!(&mut hash_hex, "{:02x}", byte).expect("Error formatting hash");
            }
            print!("\r{}", hash_hex);

            let hash_int = BigInt::from_bytes_be(num_bigint::Sign::Plus, &hash);

            if hash_int < self.target {
                println!("\n");
                break;
            }
            nonce += 1;
        }

        // nonce를 u16으로 안전하게 변환
        let final_nonce: u16 = nonce;

        (final_nonce, hash)
    }

    pub fn validate(&self) -> bool {
        let data = self.prepare_data(self.block.nonce); // nonce를 i32로 캐스팅
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize().to_vec();

        let hash_int = BigInt::from_bytes_be(num_bigint::Sign::Plus, &hash);
        hash_int < self.target
    }
}
