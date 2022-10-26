use std::collections::HashMap;

use crate::{error::BlockchainError, Block, Txoutput};

mod sleddb;

pub use sleddb::SledDb;

pub const TIP_KEY: &str = "tip_hash";
pub const HEIGHT: &str = "height";
pub const TABLE_OF_BLOCK: &str = "blocks";
pub const UTXO_SET: &str = "utxos";
pub trait Storage: Send + Sync + 'static {
    // 마지막 블록의 해시 값 가져오기
    fn get_tip(&self) -> Result<Option<String>, BlockchainError>;
    // 블록가져오기
    fn get_block(&self, key: &str) -> Result<Option<Block>, BlockchainError>;
    // 블록체인의 높이
    fn get_height(&self) -> Result<Option<usize>, BlockchainError>;
    // 트랜잭션 방식으로 블록체인 업데이트
    fn update_blocks(&self, key: &str, block: &Block, height: usize);
    // 블록의 반복자
    fn get_block_iter(&self) -> Result<Box<dyn Iterator<Item = Block>>, BlockchainError>;
    fn get_utxo_set(&self) -> HashMap<String, Vec<Txoutput>>;
    fn write_utxo(&self, txid: &str, outs: Vec<Txoutput>) -> Result<(), BlockchainError>;
    fn clear_utxo_set(&self);
}
// 블록을 정의하는 반복자
pub struct StorageIterator<T> {
    data: T,
}

impl<T> StorageIterator<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}
// T 제네릭은 Iterator 제약 조건을 충족해야 합니다.
// T의 항목 유형은 블록으로 변환될 수 있어야 합니다.
impl<T> Iterator for StorageIterator<T>
where
    T: Iterator,
    T::Item: Into<Block>,
{
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(|v| v.into())
    }
}
