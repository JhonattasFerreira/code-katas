mod bits;
mod cli;
mod decoder;
mod encoder;
mod frequency;
mod table;
mod tree;

use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let parsed = match cli::parse_args(&args) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let data = match fs::read(&parsed.input) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error: could not read file \"{}\": {}", parsed.input, e);
            std::process::exit(1);
        }
    };

    let result = match parsed.command {
        cli::Command::Compress => encoder::encode(&data),
        cli::Command::Decompress => decoder::decode(&data),
    };

    if let Err(e) = fs::write(&parsed.output, &result) {
        eprintln!("Error: could not write file \"{}\": {}", parsed.output, e);
        std::process::exit(1);
    }
}
