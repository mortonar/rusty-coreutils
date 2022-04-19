use clap::Parser;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Result, Write};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    files: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    for f in cli.files {
        let file = File::open(f)?;
        for l in BufReader::new(file).lines() {
            let _written = io::stdout().write(format!("{}\n", l.unwrap()).as_bytes())?;
        }
    }
    Ok(())
}
