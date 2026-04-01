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

    // step1 files
    #[test]
    fn test_step1_valid() {
        let input = std::fs::read_to_string("tests/step1/valid.json").unwrap();
        assert_eq!(run(&input), Ok(()));
    }

    #[test]
    fn test_step1_invalid() {
        let input = std::fs::read_to_string("tests/step1/invalid.json").unwrap();
        assert!(run(&input).is_err());
    }

    // step3 files
    #[test]
    fn test_step3_valid() {
        let input = std::fs::read_to_string("tests/step3/valid.json").unwrap();
        assert_eq!(run(&input), Ok(()));
    }

    #[test]
    fn test_step3_invalid() {
        let input = std::fs::read_to_string("tests/step3/invalid.json").unwrap();
        assert!(run(&input).is_err());
    }

    // step4 files
    #[test]
    fn test_step4_valid() {
        let input = std::fs::read_to_string("tests/step4/valid.json").unwrap();
        assert_eq!(run(&input), Ok(()));
    }

    #[test]
    fn test_step4_valid2() {
        let input = std::fs::read_to_string("tests/step4/valid2.json").unwrap();
        assert_eq!(run(&input), Ok(()));
    }

    #[test]
    fn test_step4_invalid() {
        let input = std::fs::read_to_string("tests/step4/invalid.json").unwrap();
        assert!(run(&input).is_err());
    }

    // step2 files
    #[test]
    fn test_step2_valid() {
        let input = std::fs::read_to_string("tests/step2/valid.json").unwrap();
        assert_eq!(run(&input), Ok(()));
    }

    #[test]
    fn test_step2_valid2() {
        let input = std::fs::read_to_string("tests/step2/valid2.json").unwrap();
        assert_eq!(run(&input), Ok(()));
    }

    #[test]
    fn test_step2_invalid() {
        let input = std::fs::read_to_string("tests/step2/invalid.json").unwrap();
        assert!(run(&input).is_err());
    }

    #[test]
    fn test_step2_invalid2() {
        let input = std::fs::read_to_string("tests/step2/invalid2.json").unwrap();
        assert!(run(&input).is_err());
    }
}
