#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UTXO {
    tx_id: String,
    output_index: u32,
    amount: u64,
    owner: String    //higher fee = higher tx priority
}

#[derive(Debug, Clone)]
pub struct Transaction {
    tx_id: String,
    inputs: Vec<UTXO>,
    outputs: Vec<UTXO>
}

impl Transaction {
    pub fn send() {
        
    }
}