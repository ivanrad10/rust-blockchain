use std::collections::HashMap;
use std::ptr::null;
use std::sync::{Arc, Mutex};

use crate::block::Block;
use crate::errors::Result;
use crate::transaction::{Transaction, UTXO};

const TARGET_HEXT: usize = 4;

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
    utxo_pool: Arc<Mutex<HashMap<String, Vec<UTXO>>>>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: vec![Block::new_genesis_block()],
            utxo_pool: Arc::new((Mutex::new((HashMap::new()))))
        }
    }

    pub fn add_block(&mut self, data: String) -> Result<()> {
        let prev = self.blocks.last().unwrap();
        let new_block = Block::new_block(data, prev.get_hash(), TARGET_HEXT)?;
        self.blocks.push(new_block);
        Ok(())
    }

    pub fn initiate_tx(&self, from: String, to: String, amount: u64) -> Option<Transaction> {
        let mut utxo_pool = self.utxo_pool.lock().unwrap();

        let sender_utxos = utxo_pool.get_mut(&from)?;
        let mut selected_utxos = Vec::new();
        let mut total_available = 0;

        for utxo in sender_utxos.iter() {
            selected_utxos.push(utxo.clone());
            total_available += utxo.amount;
            if total_available > amount {
                break;
            }
        }

        if total_available < amount {
            println!("❌ Insufficient balance!");
            return None;
        }

        let mut new_outputs = vec![UTXO {
            tx_id: "new_tx".to_string(), // Placeholder for a real hash
            output_index: 0,
            amount,
            owner: to.to_string(),
        }];

        // give the change back to the sender if it exists
        if total_available > amount {
            new_outputs.push(UTXO {
                tx_id: "new_tx".to_string(),
                output_index: 1,
                amount: total_available - amount,
                owner: from.to_string(),
            });
        }

        let new_transaction = Transaction {
            tx_id: "new_tx".to_string(), // TODO
            inputs: selected_utxos.clone(),
            outputs: new_outputs.clone(),
        };
    
        // Remove spent UTXOs from the pool
        let sender_utxos = utxo_pool.get_mut(&from)?;
        sender_utxos.retain(|utxo| !selected_utxos.contains(utxo)); 
    
        // Add new UTXOs to the pool
        utxo_pool.entry(to.to_string()).or_insert_with(Vec::new).extend(new_outputs.clone());
        utxo_pool.entry(from.to_string()).or_insert_with(Vec::new).extend(
            new_outputs.iter().filter(|utxo| utxo.owner == from).cloned(),
        );
    
        println!("✅ Transaction created: {:?}", new_transaction);
        Some(new_transaction)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain() {
        let mut b = Blockchain::new();

        b.add_block("data1".to_string());
        b.add_block("data2".to_string());
        b.add_block("data3".to_string());
    }
}