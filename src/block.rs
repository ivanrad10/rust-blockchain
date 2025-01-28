use std::{time::SystemTime, vec};
use clap::builder::Str;
use log::info;
use crypto::{digest::Digest, sha2::Sha256};

use crate::errors::Result;

const TARGET_HEXT: usize = 4;

#[derive(Debug, Clone)]
pub struct Block {
    timestamp: u128,
    transactions: String,
    prev_block_hash: String,
    hash: String,
    height: usize,
    nonce: i32,
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