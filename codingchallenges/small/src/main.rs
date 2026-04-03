mod bits;
mod cli;
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

    let freq = frequency::count(&data);

    for (byte, count) in freq.iter().enumerate() {
        if *count > 0 {
            let display = if (byte as u8).is_ascii_graphic() {
                format!("'{}'", byte as u8 as char)
            } else {
                format!("0x{:02X}", byte)
            };
            println!("{:>6}: {}", display, count);
        }
    }
}
