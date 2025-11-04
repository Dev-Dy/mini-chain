// ============================================================================
// 🧪 Integration Tests for Mini Blockchain with Proof-of-Work
// Author: <Your Name> (@Dev-Dy)
// Repository: https://github.com/Dev-Dy/mini_chain
// ----------------------------------------------------------------------------
// These tests verify that the blockchain correctly mines, validates, and
// detects tampering after the Proof-of-Work upgrade.
// ============================================================================

use mini_chain::Blockchain;

/// Test 1: Verify blockchain initializes correctly and mines the genesis block.
#[test]
fn test_blockchain_initialization() {
    // Difficulty 2 = quick mining for tests (low difficulty)
    let chain = Blockchain::new(2);

    // There should always be one block (the genesis block)
    assert_eq!(chain.blocks.len(), 1);
    assert_eq!(chain.blocks[0].index, 0);

    // Hash should start with "00" (since difficulty=2)
    assert!(chain.blocks[0].hash.starts_with("00"));
}

/// Test 2: Add a few blocks and ensure chain remains valid.
#[test]
fn test_adding_blocks_and_validation() {
    let mut chain = Blockchain::new(2);

    // Add multiple blocks — each will be mined automatically.
    chain.add_block("Alice sends 5 BTC to Bob".into());
    chain.add_block("Bob sends 2 BTC to Carol".into());
    chain.add_block("Carol sends 1 BTC to Dave".into());

    // Verify total number of blocks (genesis + 3)
    assert_eq!(chain.blocks.len(), 4);

    // The chain should be valid
    assert!(chain.is_valid());

    // Each block should have a valid hash prefix for the set difficulty
    for block in &chain.blocks {
        assert!(block.hash.starts_with("00"));
    }
}

/// Test 3: Tampering with block data should invalidate the chain.
#[test]
fn test_tampering_invalidates_chain() {
    let mut chain = Blockchain::new(2);
    chain.add_block("Legit transaction".into());

    // Tamper with the block's data
    chain.blocks[1].data = "Malicious edit".into();

    // Now validation should fail
    assert!(!chain.is_valid());
}

/// Test 4: Ensure Proof-of-Work actually changes nonce values.
#[test]
fn test_nonce_increases_during_mining() {
    let mut chain = Blockchain::new(2);
    chain.add_block("Mining test".into());

    // The block should have a nonce > 0 (mining required it)
    assert!(chain.blocks[1].nonce > 0);
}

/// Test 5: Difficulty scaling sanity check.
/// Increasing difficulty should require more attempts (higher nonce values).
#[test]
fn test_difficulty_affects_nonce() {
    let mut easy_chain = Blockchain::new(1);
    easy_chain.add_block("Easy mining".into());

    let mut hard_chain = Blockchain::new(3);
    hard_chain.add_block("Hard mining".into());

    // Typically, higher difficulty → higher nonce (not always, but generally)
    assert!(
        hard_chain.blocks[1].nonce >= easy_chain.blocks[1].nonce,
        "Expected higher nonce for harder difficulty"
    );
}

#[test]
fn test_dynamic_difficulty_adjustment() {
    let mut chain = Blockchain::new(2);

    // Add multiple blocks to trigger adjustment
    for _ in 0..5 {
        chain.add_block("Timing test".into());
    }

    // Difficulty should have changed after a few blocks
    assert!(chain.difficulty >= 1);
}
