use serde::{Deserialize, Serialize};

use crate::{
    hash_pub_key,
    utils::{ecdsa_p256_sha256_sign_digest, ecdsa_p256_sha256_sign_verify, hash_to_str, serialize},
    Blockchain, Storage, Txinput, Txoutput, UTXOSet, Wallets,
};

const SUBSIDY: i32 = 10;
/*
거래

id: 트랜잭션 해시 값
vin: 트랜잭션 입력 세트
vout: 트랜잭션 출력 수집
*/
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Transaction {
    id: String,
    vin: Vec<Txinput>,
    vout: Vec<Txoutput>,
}

impl Transaction {
    // 채굴 보상, 거래 입력 없음
    pub fn new_coinbase(to: &str) -> Self {
        let txin = Txinput::default();
        let txout = Txoutput::new(SUBSIDY, to);

        let mut tx = Transaction {
            id: String::new(),
            vin: vec![txin],
            vout: vec![txout],
        };
        tx.set_hash();

        tx
    }
    // 트랜잭션 생성
    pub fn new_utxo<T: Storage>(
        from: &str,
        to: &str,
        amount: i32,
        utxo_set: &UTXOSet<T>,
        bc: &Blockchain<T>,
    ) -> Self {
        //// UTXO 세트에서 사용되지 않은 트랜잭션 출력을 가져오기
        let wallets = Wallets::new().unwrap();
        let wallet = wallets.get_wallet(from).unwrap();
        let public_key_hash = hash_pub_key(wallet.get_public_key());

        let (accumulated, valid_outputs) =
            utxo_set.find_spendable_outputs(&public_key_hash, amount);
        if accumulated < amount {
            panic!("Error not enough funds");
        }
        // 트랜잭션 입력 구성
        let mut inputs = vec![];
        for (txid, outputs) in valid_outputs {
            for idx in outputs {
                let input =
                    Txinput::new(txid.clone(), idx.clone(), wallet.get_public_key().to_vec());
                inputs.push(input);
            }
        }

        // 트랜잭션 출력 빌드
        let mut outputs = vec![Txoutput::new(amount, &to)];
        if accumulated > amount {
            // 변화를 얻는다
            outputs.push(Txoutput::new(accumulated - amount, &from));
        }

        let mut tx = Transaction {
            id: String::new(),
            vin: inputs,
            vout: outputs,
        };
        tx.set_hash();
        tx.sign(bc, wallet.get_pkcs8());

        tx
    }

    fn set_hash(&mut self) {
        if let Ok(tx_ser) = serialize(self) {
            self.id = hash_to_str(&tx_ser)
        }
    }

    fn sign<T: Storage>(&mut self, bc: &Blockchain<T>, pkcs8: &[u8]) {
        let mut tx_copy = self.trimmed_copy();

        for (idx, vin) in self.vin.iter_mut().enumerate() {
            // 입력에서 참조하는 트랜잭션 찾기
            let prev_tx_option = bc.find_transaction(vin.get_txid());
            if prev_tx_option.is_none() {
                panic!("ERROR: Previous transaction is not correct")
            }
            let prev_tx = prev_tx_option.unwrap();
            tx_copy.vin[idx].set_signature(vec![]);
            tx_copy.vin[idx].set_pub_key(prev_tx.vout[vin.get_vout()].get_pub_key_hash());
            tx_copy.set_hash();

            tx_copy.vin[idx].set_pub_key(&vec![]);

            // 개인 키로 데이터 서명
            let signature = ecdsa_p256_sha256_sign_digest(pkcs8, tx_copy.id.as_bytes());
            vin.set_signature(signature);
        }
    }

    pub fn verify<T: Storage>(&self, bc: &Blockchain<T>) -> bool {
        if self.is_coinbase() {
            return true;
        }
        let mut tx_copy = self.trimmed_copy();
        for (idx, vin) in self.vin.iter().enumerate() {
            let prev_tx_option = bc.find_transaction(vin.get_txid());
            if prev_tx_option.is_none() {
                panic!("ERROR: Previous transaction is not correct")
            }
            let prev_tx = prev_tx_option.unwrap();
            tx_copy.vin[idx].set_signature(vec![]);
            tx_copy.vin[idx].set_pub_key(prev_tx.vout[vin.get_vout()].get_pub_key_hash());
            tx_copy.set_hash();

            tx_copy.vin[idx].set_pub_key(&vec![]);

            // 공개키로 서명 확인
            let verify = ecdsa_p256_sha256_sign_verify(
                vin.get_pub_key(),
                vin.get_signature(),
                tx_copy.id.as_bytes(),
            );
            if !verify {
                return false;
            }
        }
        true
    }

    /// 判断是否是 coinbase 交易
    pub fn is_coinbase(&self) -> bool {
        self.vin.len() == 1 && self.vin[0].get_pub_key().len() == 0
    }

    fn trimmed_copy(&self) -> Transaction {
        let mut inputs = vec![];
        let mut outputs = vec![];
        for input in &self.vin {
            let txinput = Txinput::new(input.get_txid(), input.get_vout(), vec![]);
            inputs.push(txinput);
        }
        for output in &self.vout {
            outputs.push(output.clone());
        }
        Transaction {
            id: self.id.clone(),
            vin: inputs,
            vout: outputs,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_vout(&self) -> &[Txoutput] {
        self.vout.as_slice()
    }

    pub fn get_vin(&self) -> &[Txinput] {
        self.vin.as_slice()
    }
}
