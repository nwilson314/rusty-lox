mod error;
mod lox;
mod scanner;
mod token;
mod token_type;

use std::env;

use crate::lox::Lox;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();
    if args.len() > 2 {
        println!("Usage: cargo run [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        lox.run_file(&args[1]);
    } else {
        lox.run_prompt();
    }
}
