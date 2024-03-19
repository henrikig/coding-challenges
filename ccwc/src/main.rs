use std::{fs, io::Read};

use anyhow::Context;
use clap::Parser;

/// A simple implementation of the `wc` (word count) utility.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of bytes in file
    #[arg(short = 'c')]
    num_bytes: bool,

    /// Number of lines in file
    #[arg(short = 'l')]
    num_lines: bool,

    /// Number of words in file
    #[arg(short = 'w')]
    num_words: bool,

    /// Number of characters in file, supporting multibyte characters
    #[arg(short = 'm')]
    num_characters: bool,

    /// File name. If no filename is provided, stdin is used.
    file: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut contents = String::new();

    if let Some(filename) = &args.file {
        fs::File::open(filename)
            .context(format!("failed to open file `{}`", filename))?
            .read_to_string(&mut contents)
            .context(format!("failed to read file `{}`", filename))?;
    } else {
        std::io::stdin()
            .read_to_string(&mut contents)
            .context("failed to read from stdin")?;
    }

    let mut output = String::new();

    if args.num_bytes {
        output.push_str(&format!("{:>8}", contents.len()));
    } else if args.num_lines {
        output.push_str(&format!("{:>8}", contents.lines().count()));
    } else if args.num_words {
        output.push_str(&format!("{:>8}", contents.split_whitespace().count()));
    } else if args.num_characters {
        output.push_str(&format!("{:>8}", contents.chars().count()));
    } else {
        output.push_str(&format!(
            "{:>8} {:>8} {:>8}",
            contents.lines().count(),
            contents.split_whitespace().count(),
            contents.chars().count()
        ));
    }

    if let Some(filename) = &args.file {
        output.push_str(&format!(" {}", filename));
    }

    println!("{}", output);

    Ok(())
}
