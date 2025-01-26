use std::{time::SystemTime, vec};
use clap::builder::Str;
use log::info;
use crypto::{digest::Digest, sha2::Sha256};

const TARGET_HEXT: usize = 4;
pub type Result<T> = std::result::Result<T, failure::Error>;

#[derive(Debug, Clone)]
pub struct Block {
    timestamp: u128,
    transactions: String,
    prev_block_hash: String,
    hash: String,
    height: usize,
    nonce: i32,
}

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>
}

impl Block {
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn new_genesis_block() -> Block {
        Block::new_block(String::from("Genesis block"), String::new(), 0).unwrap()
    }

    pub fn new_block(data: String, prev_block_hash: String, height: usize) -> Result<Block> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis();
        let mut block = Block {
            timestamp,
            transactions: data,
            prev_block_hash,
            hash: String::new(),
            height,
            nonce: 0,
        };
        block.run_proof_if_work()?;
        Ok(block)
    }

    fn run_proof_if_work(&mut self) -> Result<()> {
        info!("Mining the block");

        while !self.validate()? {
            self.nonce += 1;
        }

        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();
        Ok(())
    } 

    fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        let data_string = format!(
            "{}{}{}{}{}",
            self.prev_block_hash,
            self.transactions,
            self.timestamp,
            TARGET_HEXT,
            self.nonce
        );
        let bytes = data_string.into_bytes();
        Ok(bytes)
    }

    fn validate(&self) -> Result<bool> {
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        let target = "0".repeat(TARGET_HEXT);
        Ok(&hasher.result_str()[0..TARGET_HEXT] == target)
    }
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: vec![Block::new_genesis_block()]
        }
    }

    pub fn add_block(&mut self, data: String) -> Result<()> {
        let prev = self.blocks.last().unwrap();
        let new_block = Block::new_block(data, prev.get_hash(), TARGET_HEXT)?;
        self.blocks.push(new_block);
        Ok(())
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
        
        assert_eq!(b.blocks.len(), 4); 
        assert_eq!(b.blocks[1].transactions, "data1");
        assert_eq!(b.blocks[2].transactions, "data2");
        assert_eq!(b.blocks[3].transactions, "data3");

        assert_eq!(&b.blocks[1].hash[0..4], "0000");
        assert_eq!(&b.blocks[2].hash[0..4], "0000");
        assert_eq!(&b.blocks[3].hash[0..4], "0000");
    }
}