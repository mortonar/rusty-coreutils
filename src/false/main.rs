use clap::Parser;
use std::process;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {}

fn main() {
    process::exit(1);
}
