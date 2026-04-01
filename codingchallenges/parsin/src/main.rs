mod lexer;
mod parser;

fn run(input: &str) -> Result<(), String> {
    let tokens = lexer::tokenize(input)?;
    parser::parse(&tokens)
}

fn main() {
    let path = std::env::args().nth(1).expect("Usage: parsin <file>");
    let input = std::fs::read_to_string(&path).expect("Failed to read file");

    match run(&input) {
        Ok(()) => {
            println!("valid");
            std::process::exit(0);
        }
        Err(e) => {
            println!("invalid: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_empty_object() {
        assert_eq!(run("{}"), Ok(()));
    }

    #[test]
    fn test_empty_input() {
        assert!(run("").is_err());
    }

    #[test]
    fn test_unclosed_brace() {
        assert!(run("{").is_err());
    }
}
