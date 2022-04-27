use clap::Parser;
use std::env;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    variable: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    if cli.variable.is_empty() {
        for (var, value) in env::vars() {
            println!("{}={}", var, value);
        }
    } else {
        for var in cli.variable {
            if let Ok(value) = env::var(var) {
                println!("{}", value);
            }
        }
    }
}
