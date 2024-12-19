use bit_rs::blockchain::Blockchain; // Blockchain 모듈을 사용

fn main() {
    // 새로운 블록체인 생성 (genesis 블록 포함)
    let mut blockchain = Blockchain::new();

    // 첫 번째 블록에 거래 2개 추가
    blockchain.add_block(vec![
        "Transaction 1".to_string(), // 첫 번째 거래
        "Transaction 2".to_string(), // 두 번째 거래
    ]);

    // 두 번째 블록에 거래 1개 추가
    blockchain.add_block(vec!["Transaction 3".to_string()]); // 세 번째 거래

    // 블록체인에 포함된 모든 블록을 출력
    for block in blockchain.chain {
        println!("{:?}", block); // 각 블록을 디버그 출력
    }
}
