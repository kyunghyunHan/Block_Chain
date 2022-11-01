use bincode;
mod error;
use crypto::{digest::Digest, sha2::Sha256};
use error::BlockchainError;
use serde::{Deserialize, Serialize};
pub struct BlockHeader {
    timestamp: i64,
    prev_hash: String,
    txs_hash: String,
    bits: usize,
    nonce: usize,
}
fn main() {
    println!("hello");

    let mut test = BlockHeader {
        timestamp: 1,
        prev_hash: "2".to_string(),
        txs_hash: "2".to_string(),
        bits: 1,
        nonce: 1,
    };
}

pub fn serialize<T>(data: &T) -> Result<Vec<u8>, BlockchainError>
where
    T: Serialize + ?Sized,
{
    //Bincode는 작은 바이너리 직렬화 전략을 사용하여 인코딩 및 디코딩하기 위한 상자
    //serialize:기본 구성을 사용하여 직렬화 가능한 개체를 Vec바이트 단위로 직렬화
    Ok(bincode::serialize(data)?)
}
