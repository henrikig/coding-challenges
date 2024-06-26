use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use comprs::{decode, encode};

#[derive(Parser)]
#[command(name = "huffman")]
#[command(about = "A program to encode and decode files with Huffman encoding.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encode {
        #[arg(value_name = "SOURCE")]
        source: PathBuf,

        #[arg(value_name = "DESTINATION")]
        destination: PathBuf,
    },
    Decode {
        #[arg(value_name = "SOURCE")]
        source: PathBuf,

        #[arg(value_name = "DESTINATION")]
        destination: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode {
            source,
            destination,
        } => encode(&source, &destination),
        Commands::Decode {
            source,
            destination,
        } => decode(&source, &destination),
    }
}
