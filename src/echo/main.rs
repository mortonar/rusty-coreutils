use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    string: Vec<String>,
    /// do not output the trailing newline
    #[clap(short)]
    no_newline: bool,
}

fn main() {
    let cli = Cli::parse();
    let line = cli.string.join(" ");
    print!("{}", &line);
    if !cli.no_newline {
        println!();
    }
}
