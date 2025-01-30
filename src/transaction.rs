#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Transaction {
    id: u64,
    sender: String,
    receiver: String,
    amount: u64,
    fee: u64    //higher fee = higher tx priority
}