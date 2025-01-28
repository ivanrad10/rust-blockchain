mod block;
mod blockchain;
mod errors;

use crate::block::Block;
use crate::blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    
    blockchain.add_block("data1".to_string()).unwrap();
    blockchain.add_block("data2".to_string()).unwrap();
    blockchain.add_block("data3".to_string()).unwrap();
}
