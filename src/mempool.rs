use crate::transaction::Transaction;

pub struct Mempool {
    transactions: Vec<Transaction>,
}