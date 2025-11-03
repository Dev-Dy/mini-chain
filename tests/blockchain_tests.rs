// ============================================================================
// 🧪 Integration Tests
// ----------------------------------------------------------------------------
// These test the library from an external perspective (like an app would).
// ============================================================================

use mini_chain::{Blockchain, Block};

#[test]
fn test_add_blocks_and_validate() {
    let mut chain = Blockchain::new();
    chain.add_block("Genesis extended".into());
    chain.add_block("Transaction 2".into());
    assert!(chain.is_valid());
}

#[test]
fn test_detect_tampering() {
    let mut chain = Blockchain::new();
    chain.add_block("Transaction A".into());
    chain.blocks[1].data = "Tampered".into();
    assert!(!chain.is_valid());
}
