#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Wallet {
    pubkey: String,
    secret_key: String,
    balance: u64,
}