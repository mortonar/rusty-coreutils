use clap::Parser;
use std::fs;
use std::io::Result;
use std::process::exit;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    directories: Vec<String>,
}

fn main() -> Result<()> {
    let mut return_value = 0;
    let cli = Cli::parse();
    if cli.directories.is_empty() {
        eprintln!("mkdir: missing operand");
        eprintln!("Try 'mkdir --help' for more information.");
        exit(1);
    } else {
        for d in cli.directories {
            if let Err(e) = fs::create_dir(&d) {
                // TODO Truncate the "(os error #)" being reported on MacOS?
                eprintln!("mkdir: failed to create directory '{}': {}", &d, e);
                return_value = 1;
            }
        }
    }
    exit(return_value)
}
