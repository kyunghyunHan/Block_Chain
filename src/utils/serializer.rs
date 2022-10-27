use anyhow::Result;
use crypto::{digest::Digest, sha3::Sha3};
use serde::{Deserialize, Serialize};

use crate::error::BlockchainError;
/*
해시값 계산
블록헤더에는 모든 정보가 포함되어 있고 트랜잭션의 해시값은 나중에 블록헤더에 추가
*/
pub fn serialize<T>(data: &T) -> Result<Vec<u8>, BlockchainError>
where
    T: Serialize + ?Sized,
{
    Ok(bincode::serialize(data)?)
}

#[allow(dead_code)]
pub fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, BlockchainError>
where
    T: Deserialize<'a> + ?Sized,
{
    //serde,bincode라이브러리 사용해서 블록헤더를 바이트로 직렬화
    //SHA256사용 해시값 계산
    Ok(bincode::deserialize(data)?)
}

pub fn hash_to_str(data: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(data);
    hasher.result_str()
}

pub fn hash_to_u8(data: &[u8], out: &mut [u8]) {
    let mut hasher = Sha3::sha3_256();
    hasher.input(data);
    hasher.result(out);
}
