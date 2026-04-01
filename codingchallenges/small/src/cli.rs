#[derive(Debug)]
pub enum Command {
    Compress,
    Decompress,
}

#[derive(Debug)]
pub struct Args {
    pub command: Command,
    pub input: String,
    pub output: String,
}

pub fn parse_args(args: &[String]) -> Result<Args, String> {
    if args.len() < 2 {
        return Err("usage: ccli <compress|decompress> <input> -o <output>".to_string());
    }

    let command = match args[1].as_str() {
        "compress" => Command::Compress,
        "decompress" => Command::Decompress,
        other => {
            return Err(format!(
                "unknown command \"{}\" — expected \"compress\" or \"decompress\"",
                other
            ));
        }
    };

    if args.len() < 3 {
        return Err("missing input file".to_string());
    }

    let input = args[2].clone();

    let output = match args.iter().position(|a| a == "-o") {
        None => return Err("missing -o flag".to_string()),
        Some(pos) => match args.get(pos + 1) {
            None => return Err("missing value after -o flag".to_string()),
            Some(val) => val.clone(),
        },
    };

    Ok(Args {
        command,
        input,
        output,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_args(slice: &[&str]) -> Vec<String> {
        slice.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_compress_command_is_parsed() {
        let args = to_args(&["ccli", "compress", "input.txt", "-o", "output.huff"]);
        let result = parse_args(&args).unwrap();
        assert!(matches!(result.command, Command::Compress));
    }

    #[test]
    fn test_decompress_command_is_parsed() {
        let args = to_args(&["ccli", "decompress", "output.huff", "-o", "result.txt"]);
        let result = parse_args(&args).unwrap();
        assert!(matches!(result.command, Command::Decompress));
    }

    #[test]
    fn test_input_and_output_are_captured() {
        let args = to_args(&["ccli", "compress", "input.txt", "-o", "output.huff"]);
        let result = parse_args(&args).unwrap();
        assert_eq!(result.input, "input.txt");
        assert_eq!(result.output, "output.huff");
    }

    #[test]
    fn test_unknown_command_returns_error() {
        let args = to_args(&["ccli", "compres", "input.txt", "-o", "output.huff"]);
        let result = parse_args(&args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("compres"));
    }

    #[test]
    fn test_missing_output_flag_returns_error() {
        let args = to_args(&["ccli", "compress", "input.txt"]);
        let result = parse_args(&args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("-o"));
    }

    #[test]
    fn test_missing_output_value_returns_error() {
        let args = to_args(&["ccli", "compress", "input.txt", "-o"]);
        let result = parse_args(&args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("-o"));
    }

    #[test]
    fn test_too_few_args_returns_error() {
        let args = to_args(&["ccli"]);
        let result = parse_args(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_input_returns_error() {
        let args = to_args(&["ccli", "compress"]);
        let result = parse_args(&args);
        assert!(result.is_err());
    }
}
