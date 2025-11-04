use mini_chain::Blockchain;
use std::io::{self, Write};

fn main() {
    let mut chain = Blockchain::new(3); // start difficulty = 3

    println!("🚀 Adaptive Blockchain (Dynamic Difficulty Enabled)");
    println!("──────────────────────────────────────────────");
    println!("Commands:");
    println!("  <text> : Add new block (auto-mines + adjusts difficulty)");
    println!("  show   : Display blockchain");
    println!("  exit   : Quit\n");

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
                println!("\n📜 Blockchain State:");
                chain.print_chain();
                println!("✅ Chain valid: {}", chain.is_valid());
                println!("Current difficulty: {}\n", chain.difficulty);
            }
            _ if !input.is_empty() => {
                chain.add_block(input.into());
                println!("✅ Block added | New difficulty: {}\n", chain.difficulty);
            }
            _ => continue,
        }
    }
}
