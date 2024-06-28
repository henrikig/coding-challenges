use std::{error::Error, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use csv::StringRecord;

#[derive(Parser, Debug)]
#[command(
    name = "razor",
    version = "1.0",
    author = "Henrik Irgens Gravdal",
    about = "Cuts columns from input"
)]
struct Args {
    /// Select columns (e.g., -f1, -f1,2, -f"1 2")
    #[arg(short, long, value_name = "COLUMNS", value_parser = parse_fields)]
    fields: Vec<usize>,

    /// Delimiter
    #[arg(short, long, default_value = "\t")]
    delimiter: char,

    /// Input file
    #[arg(name = "FILE")]
    file: Option<PathBuf>,
}

fn parse_fields(s: &str) -> Result<Vec<usize>, Box<dyn Error + Send + Sync + 'static>> {
    let fields: Vec<usize> = if s.contains(',') {
        s.split(',')
            .map(|field| field.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?
    } else if s.contains(' ') {
        s.split(' ')
            .map(|field| field.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?
    } else {
        vec![s.parse::<usize>()?]
    };

    let fields = fields.iter().map(|&f| f - 1).collect::<Vec<usize>>();
    dbg!(&fields);
    Ok(fields)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut builder = csv::ReaderBuilder::new();
    builder.has_headers(false);
    builder.delimiter(args.delimiter as u8);

    match args.file {
        None => {
            let mut rdr = builder.from_reader(std::io::stdin());
            print_records(&mut rdr, &args.fields)
        }
        Some(file) if file == PathBuf::from("-") => {
            let mut rdr = builder.from_reader(std::io::stdin());
            print_records(&mut rdr, &args.fields)
        }
        Some(file) => {
            let mut rdr = builder.from_path(file)?;
            print_records(&mut rdr, &args.fields)
        }
    }?;

    Ok(())
}

fn print_records<R: std::io::Read>(
    rdr: &mut csv::Reader<R>,
    fields: &[usize],
) -> anyhow::Result<()> {
    for result in rdr.records() {
        let record: StringRecord = result.context("Failed to read record")?;

        for &field in fields {
            print!("{}", record.get(field).unwrap());
        }

        println!();
    }

    Ok(())
}
