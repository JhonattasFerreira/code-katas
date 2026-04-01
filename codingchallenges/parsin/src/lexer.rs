#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    for ch in input.chars() {
        match ch {
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            ' ' | '\t' | '\n' | '\r' => {}
            _ => return Err(format!("Unexpected character: '{}'", ch)),
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_object() {
        let result = tokenize("{}");
        assert_eq!(result, Ok(vec![Token::LeftBrace, Token::RightBrace]));
    }

    #[test]
    fn test_empty_input() {
        let result = tokenize("");
        assert_eq!(result, Ok(vec![]));
    }

    #[test]
    fn test_whitespace_is_ignored() {
        let result = tokenize("  \t\n  ");
        assert_eq!(result, Ok(vec![]));
    }

    #[test]
    fn test_unexpected_character() {
        let result = tokenize("x");
        assert!(result.is_err());
    }
}
