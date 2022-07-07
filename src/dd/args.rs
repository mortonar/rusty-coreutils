use std::env;

#[derive(Debug)]
pub struct Args {
    pub infile: Option<String>,
    pub outfile: Option<String>,
}

impl Args {
    pub fn parse() -> Args {
        let mut args = Args {
            infile: None,
            outfile: None,
        };
        for a in env::args().into_iter().skip(1) {
            let tokens: Vec<&str> = a.split('=').collect();
            if let Some(&t) = tokens.first() {
                match t {
                    "if" => {
                        args.infile = tokens.last().map(|l| l.to_string());
                    }
                    "of" => {
                        args.outfile = tokens.last().map(|l| l.to_string());
                    }
                    _ => {}
                }
            }
        }
        args
    }
}
