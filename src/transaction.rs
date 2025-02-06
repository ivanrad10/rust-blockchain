#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UTXO {
    pub tx_id: String,
    pub output_index: u32,
    pub amount: u64,
    pub owner: String    //higher fee = higher tx priority
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub tx_id: String,
    pub inputs: Vec<UTXO>,
    pub outputs: Vec<UTXO>
}

impl Transaction {
}