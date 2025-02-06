use crate::transaction::Transaction;

pub struct Mempool {
    minerId: u64,
    transactions: Vec<Transaction>,
}

impl Mempool {
}