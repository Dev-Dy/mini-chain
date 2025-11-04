use mini_chain::Blockchain;
use std::io::{self, Write};

fn main() {
    // Create blockchain with difficulty = 4 (adjustable)
    let mut chain = Blockchain::new(4);

    println!("🚀 Mini Blockchain (Proof-of-Work Enabled)");
    println!("──────────────────────────────────────────");
    println!("Commands:");
    println!("  <text> : Add new transaction block (will mine it)");
    println!("  show   : Display current blockchain");
    println!("  exit   : Exit program\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "exit" => {
                println!("👋 Exiting blockchain. Goodbye!");
                break;
            }
            "show" => {
                println!("\n📜 Current Blockchain:");
                chain.print_chain();
                println!("✅ Chain valid: {}\n", chain.is_valid());
            }
            _ if !input.is_empty() => {
                chain.add_block(input.into());
                println!("✅ Block successfully mined and added!");
            }
            _ => continue,
        }
    }
}
