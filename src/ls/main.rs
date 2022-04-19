use clap::Parser;
use std::fs;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    files: Vec<String>,
    /// do not ignore entries starting with "."
    #[clap(short, long)]
    all: bool,
}

fn main() {
    let mut cli = Cli::parse();
    if cli.files.is_empty() {
        cli.files.push(String::from("./"));
    }
    for f in cli.files {
        let paths = fs::read_dir(f).unwrap();
        let mut paths: Vec<String> = paths
            .map(|p| p.unwrap().file_name().into_string().unwrap())
            .collect();

        // TODO current and parent directory are not listed when "-a" is given
        if !cli.all {
            paths.retain(|p| !p.starts_with('.'));
        }

        println!("{}", paths.join(" "));
    }
}
