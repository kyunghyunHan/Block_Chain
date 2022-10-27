use std::{
    env::{self, current_dir},
    sync::Arc,
};

use anyhow::Result;
use block_chain::{Node, SledDb};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut path = String::from("data");
    if let Some(args) = env::args().nth(2) {
        path = args;
    }

    let path = current_dir().unwrap().join(path);
    let db = Arc::new(SledDb::new(path));
    let mut node = Node::new(db).await?;
    node.start().await?;
    Ok(())
}
