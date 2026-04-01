mod cli;
mod counter;

use std::fs::File;
use std::io::{self, BufReader, Read};
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let parsed = cli::parse_args(&args).unwrap_or_else(|e| {
        eprintln!("ccwc: {e}");
        process::exit(1);
    });

    let output = match &parsed.filename {
        Some(path) => {
            let file = File::open(path).unwrap_or_else(|e| {
                eprintln!("ccwc: {path}: {e}");
                process::exit(1);
            });
            compute_output(
                &parsed.flags,
                BufReader::new(file),
                parsed.filename.as_deref(),
            )
        }
        None => compute_output(&parsed.flags, io::stdin().lock(), None),
    }
    .unwrap_or_else(|e| {
        eprintln!("ccwc: {e}");
        process::exit(1);
    });

    println!("{output}");
}

fn compute_output(
    flags: &cli::Flags,
    reader: impl Read,
    filename: Option<&str>,
) -> io::Result<String> {
    let c = counter::count_all(reader)?;
    Ok(cli::format_counts(flags, c.lines, c.words, c.chars, c.bytes, filename))
}
