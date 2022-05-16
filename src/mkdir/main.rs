use clap::Parser;
use std::fs;
use std::io::Result;
use std::process::exit;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    directories: Vec<String>,
    #[clap(short, long)]
    parents: bool,
    #[clap(short, long)]
    verbose: bool,
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
            // TODO refactor (with decorator pattern?) so we don't have to keep checking this command line param (it won't change)
            let create_res = if cli.parents {
                fs::create_dir_all(&d)
            } else {
                fs::create_dir(&d)
            };
            if let Err(e) = create_res {
                // TODO Truncate the "(os error #)" being reported on MacOS?
                eprintln!("mkdir: failed to create directory '{}': {}", &d, e);
                return_value = 1;
            } else if cli.verbose {
                // TODO this message is different in MacOS's version of mkdir
                println!("mkdir: created directory {}", &d);
            }
        }
    }
    exit(return_value)
}
