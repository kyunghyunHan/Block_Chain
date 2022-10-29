use std::{collections::HashMap, sync::Arc};

use crate::{error::BlockchainError, Blockchain, Storage};

pub struct UTXOSet<T> {
    storage: Arc<T>,
}

impl<T: Storage> UTXOSet<T> {
    pub fn new(storage: Arc<T>) -> Self {
        Self { storage }
    }
    // 새 블록이 생성되면 UTXO 세트 인덱스를 다시 작성
    pub fn reindex(&self, bc: &Blockchain<T>) -> Result<(), BlockchainError> {
        self.storage.clear_utxo_set();
        let map = bc.find_utxo();
        for (txid, outs) in map {
            self.storage.write_utxo(&txid, outs)?;
        }
        Ok(())
    }

    // 트랜잭션 개시자가 소비할 수 있는 트랜잭션 출력을 찾기
    pub fn find_spendable_outputs(
        &self,
        public_key_hash: &[u8],
        amount: i32,
    ) -> (i32, HashMap<String, Vec<usize>>) {
        //unspent_outputs이라는 새로운 해시맵 생성
        let mut unspent_outputs = HashMap::new();
        let mut accumulated = 0;
        let utxo_set = self.storage.get_utxo_set();

        for (txid, outs) in utxo_set.iter() {
            for (idx, out) in outs.iter().enumerate() {
                if out.is_locked(public_key_hash) && accumulated < amount {
                    accumulated += out.get_value();
                    unspent_outputs
                        .entry(txid.to_string())
                        .and_modify(|v: &mut Vec<usize>| v.push(idx))
                        .or_insert(vec![idx]);
                }
            }
        }

        (accumulated, unspent_outputs)
    }
}
