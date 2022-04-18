use clap::Parser;
use std::fs;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    files: Vec<String>,
}

fn main() {
    let mut cli = Cli::parse();
    if cli.files.is_empty() {
        cli.files.push(String::from("./"));
    }
    for f in cli.files {
        let paths = fs::read_dir(f).unwrap();
        let paths: Vec<String> = paths.map(|p| p.unwrap().file_name().into_string().unwrap())
            .collect();
        println!("{}", paths.join(" "));
    }
}
