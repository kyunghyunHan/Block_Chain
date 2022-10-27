use crate::{
    utils::{hash_to_str, hash_to_u8, serialize},
    Block,
};
use anyhow::Result;
use bigint::U256;
use std::ops::Shl;

const MAX_NONCE: usize = usize::MAX;
//pow구조
pub struct ProofOfWork {
    target: U256,
}

impl ProofOfWork {
    //여기서 bigint 라이브러리의 U256이 사용되며 처음에는 1로 초기화
    pub fn new(bits: usize) -> Self {
        let mut target = U256::from(1 as usize);
        //1을 왼쪽으로 256비트 시프트하고 비트가 8이면 왼쪽으로 248비트 시프트
        target = target.shl(256 - bits);

        Self { target }
    }

    pub fn run(&self, block: &mut Block) {
        let mut nonce = 0;
        //계산 오버플로를 피하기 위해 MAX_NONCE를 사용하여 size::MAX로 설정
        while nonce < MAX_NONCE {
            if let Ok(pre_hash) = Self::prepare_data(block, nonce) {
                let mut hash_u: [u8; 32] = [0; 32];
                hash_to_u8(&pre_hash, &mut hash_u);
                let pre_hash_int = U256::from(hash_u);
                // 계산된 해시 값이 목표보다 작으면 조건을 만족하고 루프를 점프 아웃한다.
                // 그렇지 않으면 nonce가 1씩 증가하고 다음 해시 계산이 입력됩니다.
                if pre_hash_int.lt(&(self.target)) {
                    block.set_hash(hash_to_str(&pre_hash));
                    break;
                } else {
                    nonce += 1;
                }
            }
        }
    }
    // 블록 헤더를 직렬화하기 위해 nonce 값을 설정
    fn prepare_data(block: &mut Block, nonce: usize) -> Result<Vec<u8>> {
        block.set_nonce(nonce);
        Ok(serialize(&(block.get_header()))?)
    }
}
