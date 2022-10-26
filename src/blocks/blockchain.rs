use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, RwLock,
};

use tracing::info;

use crate::{Block, SledDb, Storage};
//8로 하드코딩
///계산된 블록 해시 값의 처음 비트는 0
///
pub const CURR_BITS: usize = 8;
//블록:블록컨벤션
//height:블록체인의 높이,블록의 수

// 기본적으로 sled 데이터베이스 사용
pub struct Blockchain<T = SledDb> {
    storage: T,
    tip: Arc<RwLock<String>>,
    height: AtomicUsize,
}

impl<T: Storage> Blockchain<T> {
    pub fn new(storage: T) -> Self {
        // 데이터베이스에 tip 값이 있으면 메모리에 로드합니다.
        // 그렇지 않으면 제네시스 블록을 생성하고 데이터베이스에 업데이트합니다.
        if let Ok(Some(tip)) = storage.get_tip() {
            let height = storage.get_height().unwrap();
            Self {
                storage,
                tip: Arc::new(RwLock::new(tip)),
                height: AtomicUsize::new(height.unwrap()),
            }
        } else {
            let genesis_block = Block::create_genesis_block(CURR_BITS);
            let hash = genesis_block.get_hash();
            storage.update_blocks(&hash, &genesis_block, 0 as usize);

            Self {
                storage,
                tip: Arc::new(RwLock::new(hash)),
                height: AtomicUsize::new(0),
            }
        }
    }
    //블록을 체인에 추가
    pub fn mine_block(&mut self, data: &str) {
        let block = Block::new(data, &self.tip.read().unwrap(), CURR_BITS);
        let hash = block.get_hash();
        self.height.fetch_add(1, Ordering::Relaxed);
        self.storage
            .update_blocks(&hash, &block, self.height.load(Ordering::Relaxed));

        let mut tip = self.tip.write().unwrap();
        *tip = hash;
    } // 블록 가입
    pub fn blocks_info(&self) {
        let blocks = self.storage.get_block_iter().unwrap();
        for block in blocks {
            info!("{:#?}", block);
        }
    }
}
