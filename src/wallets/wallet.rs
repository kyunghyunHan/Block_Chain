use crate::utils::{base58_encode, new_private_key, ripemd160_digest, sha256_digest};
use ring::signature::{EcdsaKeyPair, KeyPair, ECDSA_P256_SHA256_FIXED_SIGNING};
use serde::{Deserialize, Serialize};

const VERSION: u8 = 0x00;
pub const ADDRESS_CHECKSUM_LEN: usize = 4;
/*
지갑
pkcs8: 개인 키
public_key: 공개 키
*/
#[derive(Serialize, Deserialize, Clone)]
pub struct Wallet {
    pkcs8: Vec<u8>,
    public_key: Vec<u8>,
}

impl Wallet {
    pub fn new() -> Self {
        let pkcs8 = new_private_key();
        let key_pair =
            EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8.as_ref()).unwrap();
        let public_key = key_pair.public_key().as_ref().to_vec();

        Self { pkcs8, public_key }
    }

    pub fn get_address(&self) -> String {
        let pub_key_hash = hash_pub_key(self.public_key.as_slice());
        let mut payload = vec![];
        //version+PubKeyHash 조합에 체크섬을 추가
        payload.push(VERSION);
        payload.extend(pub_key_hash.as_slice());
        let checksum = checksum(payload.as_slice());
        payload.extend(checksum.as_slice());
        //Base58을 사용하여 버전+PubKeyHash+체크섬 조합을 인코딩
        base58_encode(payload.as_slice())
    }

    pub fn get_pkcs8(&self) -> &[u8] {
        self.pkcs8.as_slice()
    }

    pub fn get_public_key(&self) -> &[u8] {
        self.public_key.as_slice()
    }
}
// SHA256을 사용하여 공개 키를 한 번 해시하고 RIPEMD160을 사용하여 결과를 두 번 해시
pub fn hash_pub_key(pub_key: &[u8]) -> Vec<u8> {
    //SHA256을 사용하여 이전 단계에서 생성된 결과를 두 번 해시합니다. 결과의 처음 4바이트를 체크섬으로 사용
    let pub_key_sha256 = sha256_digest(pub_key);
    let pub_key_ripemd160 = ripemd160_digest(&pub_key_sha256);
    pub_key_ripemd160
}

pub fn checksum(payload: &[u8]) -> Vec<u8> {
    let first_sha = sha256_digest(payload);
    let second_sha = sha256_digest(&first_sha);
    second_sha[0..ADDRESS_CHECKSUM_LEN].to_vec()
}
//Wallet 구조를 정의하고 Wallet을 로컬 파일 wallet.dat에 저장
