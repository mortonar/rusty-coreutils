extern crate rev_lines;
use clap::Parser;
use rev_lines::RevLines;
use std::fs::File;
use std::io::BufReader;
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
    // TODO tail apparently takes in some single character values for the line count
    let line_count = if let Some(line_count_str) = cli.lines {
        line_count_str.parse::<usize>().unwrap_or_else(|_error| {
            eprintln!("tail: {}: invalid number of lines", line_count_str);
            exit(1);
        })
    } else {
        10
    };
    if cli.file.is_empty() {
        // TODO support tailing stdin
        eprintln!("tail on stdin not supported yet!");
        exit(1);
    } else if cli.file.len() == 1 {
        let rev_lines = RevLines::new(BufReader::new(File::open(&cli.file[0]).unwrap())).unwrap();
        print_lines(rev_lines, line_count);
    } else {
        for f in cli.file {
            if let Ok(file) = File::open(&f) {
                if !cli.silent {
                    println!("==> {} <==", &f);
                }
                print_lines(RevLines::new(BufReader::new(file)).unwrap(), line_count);
            }
        }
    }
}

fn print_lines(rev_lines: RevLines<File>, line_count: usize) {
    let lines: Vec<String> = rev_lines.take(line_count).collect();
    for l in lines.iter().rev() {
        println!("{}", l);
    }
}
