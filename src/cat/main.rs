use clap::Parser;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Result, Write};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    files: Vec<String>,
    /// number all output lines
    #[clap(short, long)]
    number: bool,
    /// number nonempty output lines, overrides -n
    #[clap(short = 'b', long = "number-nonblank")]
    number_nonblank: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut writeable: Box<dyn Writeable> = if cli.number_nonblank {
        Box::new(NonBlankNumberedLineWriter {
            writer: NumberedLineWriter::new(),
        })
    } else if cli.number {
        Box::new(NumberedLineWriter::new())
    } else {
        Box::new(StandardLineWriter {})
    };

    for f in cli.files {
        let file = File::open(f)?;
        for l in BufReader::new(file).lines() {
            // TODO neatly "column-align" the number and file line columns (e.g. What if the file is thousands of lines long?)
            // TODO fix errors thrown when pipe is broken (e.g. cat -n <file> | head)
            writeable.write(l.unwrap());
        }
    }

    Ok(())
}

trait Writeable {
    fn write(&mut self, line: String);
}

struct StandardLineWriter;

impl Writeable for StandardLineWriter {
    fn write(&mut self, line: String) {
        let _written = io::stdout()
            .write(format!("{}\n", line).as_bytes())
            .unwrap();
    }
}

struct NumberedLineWriter {
    line_number: u64,
}

impl NumberedLineWriter {
    fn new() -> Self {
        NumberedLineWriter { line_number: 1 }
    }
}

impl Writeable for NumberedLineWriter {
    fn write(&mut self, line: String) {
        let _written = io::stdout()
            .write(format!("  {}  {}\n", self.line_number, line).as_bytes())
            .unwrap();
        self.line_number += 1;
    }
}

struct NonBlankNumberedLineWriter {
    writer: NumberedLineWriter,
}

impl Writeable for NonBlankNumberedLineWriter {
    fn write(&mut self, line: String) {
        if line.is_empty() {
            let _written = io::stdout().write("\n".as_bytes()).unwrap();
        } else {
            self.writer.write(line);
        }
    }
}
