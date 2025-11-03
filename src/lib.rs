// ============================================================================
// 🦀 Mini Blockchain Library in Rust
// Author: <Your Name> (@Dev-Dy)
// Repository: https://github.com/Dev-Dy/mini_chain
// ----------------------------------------------------------------------------
// This module defines the core blockchain data structures and logic.
//
// Components:
//   • Block: represents an individual unit of data.
//   • Blockchain: manages the ordered chain of blocks.
//   • Tests: validate functionality and tamper-resistance.
//
// This library can be imported by the CLI app or used independently
// in other Rust binaries or unit tests.
// ============================================================================

use sha2::{Sha256, Digest};
use chrono::Utc;
use serde::{Serialize, Deserialize};

// ============================================================================
// 🧱 BLOCK STRUCTURE
// ----------------------------------------------------------------------------
// Each block contains:
//   - index: sequential number
//   - timestamp: creation time
//   - data: payload (e.g., transactions)
//   - previous_hash: links to previous block
//   - hash: SHA-256 of the block contents
// ============================================================================
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    /// Create a new block with auto-calculated hash.
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    /// Calculate SHA-256 hash based on current block contents.
    pub fn calculate_hash(&self) -> String {
        let record = format!(
            "{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash
        );
        let mut hasher = Sha256::new();
        hasher.update(record.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

// ============================================================================
// 🔗 BLOCKCHAIN STRUCTURE
// ----------------------------------------------------------------------------
// Manages the vector of blocks, ensures hash linkage and integrity checks.
// ============================================================================
#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    /// Initialize a new blockchain with a genesis block.
    pub fn new() -> Self {
        let mut chain = Blockchain { blocks: Vec::new() };
        chain.create_genesis_block();
        chain
    }

    /// Create the genesis (first) block manually.
    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, "Genesis Block".into(), "0".into());
        self.blocks.push(genesis_block);
    }

    /// Add a new block with given data.
    pub fn add_block(&mut self, data: String) {
        let previous_block = self.blocks.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.hash.clone(),
        );
        self.blocks.push(new_block);
    }

    /// Verify if the blockchain remains valid (no tampering).
    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];

            // Ensure hashes match recalculated values.
            if current.hash != current.calculate_hash() {
                return false;
            }

            // Ensure linkage between previous and current blocks.
            if current.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }

    /// Display all blocks in readable debug format.
    pub fn print_chain(&self) {
        for block in &self.blocks {
            println!("{:#?}", block);
        }
    }
}

// ============================================================================
// 🧪 UNIT TESTS
// ----------------------------------------------------------------------------
// Tests verify chain validity and tamper detection.
// Run using: `cargo test`
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_validity() {
        let mut chain = Blockchain::new();
        chain.add_block("Alice sends 5 BTC to Bob".into());
        chain.add_block("Bob sends 2 BTC to Carol".into());
        assert!(chain.is_valid());
    }

    #[test]
    fn test_tampering_invalidates_chain() {
        let mut chain = Blockchain::new();
        chain.add_block("Legit transaction".into());
        chain.blocks[1].data = "Tampered data".into();
        assert!(!chain.is_valid());
    }
}
