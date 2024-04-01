use std::path::PathBuf;

use anyhow::{Context, Result};
use jsonrs::{lexer::Lexer, parser::Parser};

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to JSON file
    name: PathBuf,
}

fn main() -> Result<()> {
    let cli = <Cli as clap::Parser>::parse();

    let contents = std::fs::read_to_string(&cli.name)
            .context(format!("failed to open file `{:?}`", &cli.name))?;

    let mut lexer = Lexer::new(contents);
    let mut parser = Parser::new(&mut lexer);

    parser.parse()?;
    println!("Successfully parsed JSON.");
    Ok(())
}
