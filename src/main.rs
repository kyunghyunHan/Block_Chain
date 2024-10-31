use bit_rs::blockchain::Blockchain;
fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block(vec![
        "Transaction 1".to_string(),
        "Transaction 2".to_string(),
    ]);
    blockchain.add_block(vec!["Transaction 3".to_string()]);
    for block in blockchain.chain {
        println!("{:?}", block);
    }
}
