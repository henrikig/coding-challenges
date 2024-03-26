use std::path::PathBuf;

use clap::Parser;
use jsonrs::{lexer::Lexer, token::TokenType};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to JSON file
    name: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(config_path) = cli.name {
        let contents = std::fs::read_to_string(config_path).unwrap();
        let mut lexer = Lexer::new(contents);

        let mut token = lexer.next_token();
        println!("{:?}", token);

        while token.token_type != TokenType::EOF {
            token = lexer.next_token();
            println!("{:?}", token);
        }
    }
}
