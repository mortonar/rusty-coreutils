use clap::Parser;
use std::fs::File;
use std::io::{self, stdin, BufRead};
use std::process::exit;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    file: Vec<String>,
    #[clap(short = 'n', long = "lines")]
    lines: Option<String>,
    // TODO "silent" should also be accepted
    #[clap(short = 'q', long = "quiet")]
    silent: bool,
}

fn main() {
    let cli = Cli::parse();
    let line_count = if let Some(line_count_str) = cli.lines {
        line_count_str.parse::<usize>().unwrap_or_else(|_error| {
            println!("head: illegal line count -- {}", line_count_str);
            exit(1);
        })
    } else {
        10
    };

    if cli.file.len() <= 1 {
        let buf_reader: Box<dyn BufRead> = if cli.file.is_empty() {
            Box::new(io::BufReader::new(stdin()))
        } else {
            Box::new(io::BufReader::new(File::open(&cli.file[0]).unwrap()))
        };
        for line in buf_reader.lines().take(line_count) {
            println!("{}", line.unwrap());
        }
    } else {
        for f in cli.file {
            if let Ok(file) = File::open(&f) {
                if !cli.silent {
                    println!("==> {} <==", &f);
                }
                for line in io::BufReader::new(file).lines().take(line_count) {
                    println!("{}", line.unwrap());
                }
            }
        }
    }
}
