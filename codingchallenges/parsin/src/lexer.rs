#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    Colon,
    Comma,
    String(std::string::String),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            ':' => tokens.push(Token::Colon),
            ',' => tokens.push(Token::Comma),
            '"' => {
                let mut value = std::string::String::new();
                loop {
                    match chars.next() {
                        Some('"') => break,
                        Some(c) => value.push(c),
                        None => return Err(String::from("Unterminated string")),
                    }
                }
                tokens.push(Token::String(value));
            }
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

    #[test]
    fn test_colon() {
        let result = tokenize(":");
        assert_eq!(result, Ok(vec![Token::Colon]));
    }

    #[test]
    fn test_string() {
        let result = tokenize("\"hello\"");
        assert_eq!(result, Ok(vec![Token::String(String::from("hello"))]));
    }

    #[test]
    fn test_unclosed_string() {
        let result = tokenize("\"hello");
        assert!(result.is_err());
    }

    #[test]
    fn test_full_key_value_object() {
        let result = tokenize("{\"key\": \"value\"}");
        assert_eq!(
            result,
            Ok(vec![
                Token::LeftBrace,
                Token::String(String::from("key")),
                Token::Colon,
                Token::String(String::from("value")),
                Token::RightBrace,
            ])
        );
    }
}
