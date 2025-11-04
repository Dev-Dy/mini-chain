// ============================================================================
// 🦀 Mini Blockchain Library in Rust (with Proof-of-Work)
// Author: <Your Name> (@Dev-Dy)
// Repository: https://github.com/Dev-Dy/mini_chain
// ----------------------------------------------------------------------------
// Adds Proof-of-Work (PoW) mining to the basic blockchain implementation.
// Blocks now require computational work to be mined before inclusion.
// ============================================================================

use sha2::{Sha256, Digest};
use chrono::{Utc, Duration};
use serde::{Serialize, Deserialize};

// ============================================================================
// 🧱 BLOCK STRUCTURE
// ----------------------------------------------------------------------------
// Each block now includes a nonce for Proof-of-Work mining.
// ============================================================================
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64, // mining iteration counter
}

impl Block {
    /// Creates a new, *unmined* block (hash and nonce will be set during mining).
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        }
    }

    /// Calculates SHA-256 hash from the block's fields + nonce.
    pub fn calculate_hash(&self) -> String {
        let record = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(record.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Proof-of-Work: repeatedly hashes until hash meets difficulty requirement.
    /// Difficulty = number of leading zeros required in the hash.
    pub fn mine_block(&mut self, difficulty: usize) {
        println!("⛏️  Mining block {}...", self.index);

        let target_prefix = "0".repeat(difficulty);
        let start_time = Utc::now();

        loop {
            self.hash = self.calculate_hash();

            // If hash starts with N leading zeros, it's valid.
            if self.hash.starts_with(&target_prefix) {
                let duration = Utc::now() - start_time;
                println!(
                    "✅ Block mined! Hash: {}\n⏱️  Time: {}s | Nonce: {}\n",
                    self.hash,
                    duration.num_seconds(),
                    self.nonce
                );
                break;
            }

            self.nonce += 1;
            // Optional optimization: print progress every 100k attempts
            if self.nonce % 100_000 == 0 {
                print!(".");
                use std::io::Write;
                std::io::stdout().flush().unwrap();
            }
        }
    }
}

// ============================================================================
// 🔗 BLOCKCHAIN STRUCTURE (WITH MINING)
// ============================================================================
#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: usize, // controls mining difficulty
}

impl Blockchain {
    /// Creates a new blockchain with a genesis block and difficulty.
    pub fn new(difficulty: usize) -> Self {
        let mut chain = Blockchain {
            blocks: Vec::new(),
            difficulty,
        };
        chain.create_genesis_block();
        chain
    }

    /// Creates the genesis (first) block and mines it.
    fn create_genesis_block(&mut self) {
        let mut genesis_block = Block::new(0, "Genesis Block".into(), "0".into());
        genesis_block.mine_block(self.difficulty);
        self.blocks.push(genesis_block);
    }

    /// Adds and mines a new block with the given data.
    pub fn add_block(&mut self, data: String) {
        let previous_hash = self.blocks.last().unwrap().hash.clone();
        let mut new_block = Block::new(self.blocks.len() as u64, data, previous_hash);
        new_block.mine_block(self.difficulty);
        self.blocks.push(new_block);
    }

    /// Verifies that all blocks and links are valid.
    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];

            if current.hash != current.calculate_hash() {
                return false;
            }
            if current.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }

    /// Prints all blocks in the chain.
    pub fn print_chain(&self) {
        for block in &self.blocks {
            println!("{:#?}", block);
        }
    }
}
