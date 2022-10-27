use serde::{Deserialize, Serialize};

use crate::{utils::base58_decode, ADDRESS_CHECKSUM_LEN};
/*
트랜잭션 출력
가치: 거래 가치
to_addr: 트랜잭션 수신자로 다음 부분에서 지갑 기능이 구현되면 트랜잭션 수신자의 공개 키 해시로 대체
*/
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Txoutput {
    value: i32,
    pub_key_hash: Vec<u8>,
}

impl Txoutput {
    pub fn new(value: i32, to_addr: &str) -> Self {
        let mut output = Txoutput {
            value,
            pub_key_hash: vec![],
        };
        output.lock(to_addr);
        output
    }

    fn lock(&mut self, address: &str) {
        let payload = base58_decode(address);
        let pub_key_hash = payload[1..payload.len() - ADDRESS_CHECKSUM_LEN].to_vec();
        self.pub_key_hash = pub_key_hash
    }

    pub fn is_locked(&self, pub_key_hash: &[u8]) -> bool {
        self.pub_key_hash.eq(pub_key_hash)
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn get_pub_key_hash(&self) -> &[u8] {
        self.pub_key_hash.as_slice()
    }
}
