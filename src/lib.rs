mod blocks;
mod error;
mod storage;
mod transactions;
mod utils;

pub use blocks::*;
pub use storage::*;
pub use transactions::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
