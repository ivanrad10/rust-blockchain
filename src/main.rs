mod block;
mod blockchain;
mod errors;
mod mempool;
mod miner;
mod transaction;
mod wallet;

use crate::block::Block;
use crate::blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    
    blockchain.add_block("data1".to_string()).unwrap();
    blockchain.add_block("data2".to_string()).unwrap();
    blockchain.add_block("data3".to_string()).unwrap();
}
