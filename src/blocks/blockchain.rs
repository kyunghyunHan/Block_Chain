use std::{
    collections::HashMap,
    //스레드 간의 기본 공유 메모리 통신을 제공하며 다른 동시 유형의 빌딩 블록
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, RwLock,
    },
};
//범위가 지정된 구조화된 로깅 및 진단 시스템
//
use tracing::info;

use crate::{error::BlockchainError, Block, SledDb, Storage, Transaction, Txoutput};
//비트는 8비트로 하드코딩,계산된 블록 해시값의 처음 비트는 0
//이 블록은 블록체인에 추가될수 있다.
pub const CURR_BITS: usize = 8;
//블록을 메모리 저장후 나중에 데이터베이스에 저장
//블록:블록컬렉션
//블록체인의높이,블록의 수
//sled 데이터베이스를 사용
#[derive(Debug, Default)]
pub struct Blockchain<T = SledDb> {
    storage: Arc<T>,
    tip: Arc<RwLock<String>>,
    height: AtomicUsize,
}

impl<T: Storage> Blockchain<T> {
    pub fn new(storage: Arc<T>) -> Self {
        // 데이터베이스에 tip 값이 있으면 메모리에 로드
        // 그렇지 않으면 제네시스 블록을 생성하고 데이터베이스에 업데이트
        if let Ok(Some(tip)) = storage.get_tip() {
            let height = storage.get_height().unwrap();
            Self {
                storage,
                tip: Arc::new(RwLock::new(tip)),
                height: AtomicUsize::new(height.unwrap()),
            }
        } else {
            Self {
                storage,
                tip: Arc::new(RwLock::new(String::new())),
                height: AtomicUsize::new(0),
            }
        }
    }
    //제네시스 블록 생성
    pub fn create_genesis_block(&mut self, genesis_addr: &str) {
        let genesis_block = Block::create_genesis_block(CURR_BITS, genesis_addr);
        let hash = genesis_block.get_hash();
        self.height.fetch_add(1, Ordering::Relaxed);
        self.storage
            .update_blocks(&hash, &genesis_block, self.height.load(Ordering::Relaxed));
        let mut tip = self.tip.write().unwrap();
        *tip = hash;
    }
    //블록체인에 추가
    pub fn mine_block(&mut self, txs: &[Transaction]) -> Block {
        for tx in txs {
            if tx.verify(self) == false {
                panic!("ERROR: Invalid transaction")
            }
        }

        let block = Block::new(txs, &self.tip.read().unwrap(), CURR_BITS);
        //블록의 해시값 가져오기
        let hash = block.get_hash();
        //현재 값에 더하여 이전 값을 반환합니다.
        self.height.fetch_add(1, Ordering::Relaxed);
        self.storage
            .update_blocks(&hash, &block, self.height.load(Ordering::Relaxed));
        let mut tip = self.tip.write().unwrap();
        *tip = hash;

        block
    }
    //블록추가
    pub fn add_block(&mut self, block: Block) -> Result<(), BlockchainError> {
        let hash = block.get_hash();
        if let Some(_) = self.storage.get_block(&hash)? {
            info!("Block {} already exists", hash);
        } else {
            self.height.fetch_add(1, Ordering::Relaxed);
            self.storage
                .update_blocks(&hash, &block, self.height.load(Ordering::Relaxed));
            let mut tip = self.tip.write().unwrap();
            *tip = hash;
        }
        Ok(())
    }
    //찾기
    pub fn find_utxo(&self) -> HashMap<String, Vec<Txoutput>> {
        let mut utxo = HashMap::new();
        let mut spent_txos = HashMap::new();

        let blocks = self.storage.get_block_iter().unwrap();
        for block in blocks {
            for tx in block.get_tranxs() {
                for (idx, txout) in tx.get_vout().iter().enumerate() {
                    if let Some(outs) = spent_txos.get(&tx.get_id()) {
                        for out in outs {
                            if idx.eq(out) {
                                break;
                            }

                            utxo.entry(tx.get_id())
                                .and_modify(|v: &mut Vec<Txoutput>| v.push(txout.clone()))
                                .or_insert(vec![txout.clone()]);
                        }
                    } else {
                        utxo.entry(tx.get_id())
                            .and_modify(|v: &mut Vec<Txoutput>| v.push(txout.clone()))
                            .or_insert(vec![txout.clone()]);
                    }
                }

                for txin in tx.get_vin() {
                    spent_txos
                        .entry(txin.get_txid())
                        .and_modify(|v: &mut Vec<usize>| v.push(txin.get_vout()))
                        .or_insert(vec![txin.get_vout()]);
                }
            }
        }

        utxo
    }
    //트랜잭션 찾기
    pub fn find_transaction(&self, txid: String) -> Option<Transaction> {
        let blocks = self.storage.get_block_iter().unwrap();
        for block in blocks {
            for tx in block.get_tranxs() {
                if tx.get_id() == txid {
                    return Some(tx);
                }
            }
        }
        None
    }
    //블록 정보
    pub fn blocks_info(&self) {
        let blocks = self.storage.get_block_iter().unwrap();
        for block in blocks {
            info!("{:#?}", block);
        }
    }
    //블록가져오기
    pub fn get_blocks(&self) -> Vec<Block> {
        self.storage.get_block_iter().unwrap().collect()
    }

    pub fn get_tip(&self) -> String {
        self.tip.read().unwrap().to_string()
    }

    pub fn get_height(&self) -> usize {
        self.height.load(Ordering::Relaxed)
    }
}
