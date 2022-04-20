use clap::Parser;
use std::io::{self, ErrorKind, Result, Write};

// TODO allow for only one word param (expletive) for BSD

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    string: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let to_write = if !cli.string.is_empty() {
        cli.string.join(" ") + "\n"
    } else {
        String::from("y\n")
    };
    let to_write = to_write.as_bytes();
    loop {
        if let Err(e) = io::stdout().write(to_write) {
            if e.kind() == ErrorKind::BrokenPipe {
                return Ok(());
            }
            return Err(e);
        }
    }
}
