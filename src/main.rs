use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let file = File::open(&args.path)
        .expect("could not read file");
    let mut reader = BufReader::new(file);

    let mut line = String::new();

    loop {
        line.clear();
        let len = reader.read_line(&mut line);
        match len {
            Ok(0) => break,
            Ok(_) => {
                if line.contains(&args.pattern) {
                    print!("{}", line);
                }
            }
            Err(_) => break,
        }
    }
}
