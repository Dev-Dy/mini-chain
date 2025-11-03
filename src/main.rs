// ============================================================================
// 🧩 Command-Line Interface (CLI)
// ----------------------------------------------------------------------------
// This file runs the interactive blockchain demo.
// It imports core logic from the library (lib.rs) and provides
// a simple terminal interface for adding, viewing, and validating blocks.
// ============================================================================

use mini_chain::{Blockchain, Block};
use std::io::{self, Write};

fn main() {
    // Initialize blockchain
    let mut chain = Blockchain::new();

    // Display CLI banner
    println!("🚀 Mini Blockchain Started");
    println!("────────────────────────────");
    println!("Commands:");
    println!("  <text> : Add a new transaction block");
    println!("  show   : Display the current blockchain");
    println!("  exit   : Quit the program\n");

    // REPL loop for user input
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            // Exit program
            "exit" => {
                println!("👋 Exiting blockchain. Goodbye!");
                break;
            }

            // Display the blockchain contents
            "show" => {
                println!("\n📜 Current Blockchain:");
                chain.print_chain();
                println!("✅ Chain valid: {}\n", chain.is_valid());
            }

            // Add a new block with user-supplied text
            _ if !input.is_empty() => {
                chain.add_block(input.into());
                println!("✅ Block added successfully!");
            }

            // Ignore empty inputs
            _ => continue,
        }
    }
}
