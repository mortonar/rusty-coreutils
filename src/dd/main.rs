use std::fs::File;
use std::io::{stdin, stdout, Read, Write};

mod args;

fn main() {
    let args = args::Args::parse();
    let mut infile: Box<dyn Read> = if let Some(file_path) = args.infile {
        Box::new(File::open(file_path).unwrap())
    } else {
        Box::new(stdin().lock())
    };
    let mut outfile: Box<dyn Write> = if let Some(file_path) = args.outfile {
        Box::new(File::open(file_path).unwrap())
    } else {
        Box::new(stdout().lock())
    };
    let mut buf = [0; 512];
    let mut total = 0;
    while let Ok(num_read) = infile.read(&mut buf[..]) {
        if num_read == 0 {
            break;
        }
        total += num_read;
        outfile.write_all(&buf).expect("write failed!");
    }
    println!("{} bytes transferred", total);
}
