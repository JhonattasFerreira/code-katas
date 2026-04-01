#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    String(std::string::String),
    Bool(bool),
    Null,
    Number(f64),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            '[' => tokens.push(Token::LeftBracket),
            ']' => tokens.push(Token::RightBracket),
            ':' => tokens.push(Token::Colon),
            ',' => tokens.push(Token::Comma),
            '"' => tokens.push(read_string(&mut chars)?),
            't' | 'f' | 'n' => tokens.push(read_keyword(ch, &mut chars)?),
            '0'..='9' | '-' => tokens.push(read_number(ch, &mut chars)?),
            ' ' | '\t' | '\n' | '\r' => {}
            _ => return Err(format!("Unexpected character: '{}'", ch)),
        }
    }

    Ok(tokens)
}

fn read_string(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Token, String> {
    let mut value = std::string::String::new();
    loop {
        match chars.next() {
            Some('"') => return Ok(Token::String(value)),
            Some(c) => value.push(c),
            None => return Err(String::from("Unterminated string")),
        }
    }
}

fn read_keyword(
    first: char,
    chars: &mut std::iter::Peekable<std::str::Chars>,
) -> Result<Token, String> {
    let mut keyword = std::string::String::from(first);
    while let Some(&c) = chars.peek() {
        if c.is_alphabetic() {
            keyword.push(c);
            chars.next();
        } else {
            break;
        }
    }
    match keyword.as_str() {
        "true" => Ok(Token::Bool(true)),
        "false" => Ok(Token::Bool(false)),
        "null" => Ok(Token::Null),
        _ => Err(format!("Unknown keyword: '{}'", keyword)),
    }
}

fn read_number(
    first: char,
    chars: &mut std::iter::Peekable<std::str::Chars>,
) -> Result<Token, String> {
    let mut number = std::string::String::from(first);
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() || c == '.' {
            number.push(c);
            chars.next();
        } else {
            break;
        }
    }
    match number.parse::<f64>() {
        Ok(n) => Ok(Token::Number(n)),
        Err(_) => Err(format!("Invalid number: '{}'", number)),
    }
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
    fn test_comma() {
        let result = tokenize(",");
        assert_eq!(result, Ok(vec![Token::Comma]));
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
    fn test_true() {
        let result = tokenize("true");
        assert_eq!(result, Ok(vec![Token::Bool(true)]));
    }

    #[test]
    fn test_false() {
        let result = tokenize("false");
        assert_eq!(result, Ok(vec![Token::Bool(false)]));
    }

    #[test]
    fn test_null() {
        let result = tokenize("null");
        assert_eq!(result, Ok(vec![Token::Null]));
    }

    #[test]
    fn test_number_integer() {
        let result = tokenize("101");
        assert_eq!(result, Ok(vec![Token::Number(101.0)]));
    }

    #[test]
    fn test_number_float() {
        let result = tokenize("1.5");
        assert_eq!(result, Ok(vec![Token::Number(1.5)]));
    }

    #[test]
    fn test_number_negative() {
        let result = tokenize("-5");
        assert_eq!(result, Ok(vec![Token::Number(-5.0)]));
    }

    #[test]
    fn test_left_bracket() {
        let result = tokenize("[");
        assert_eq!(result, Ok(vec![Token::LeftBracket]));
    }

    #[test]
    fn test_right_bracket() {
        let result = tokenize("]");
        assert_eq!(result, Ok(vec![Token::RightBracket]));
    }

    #[test]
    fn test_empty_array() {
        let result = tokenize("[]");
        assert_eq!(result, Ok(vec![Token::LeftBracket, Token::RightBracket]));
    }

    #[test]
    fn test_unknown_keyword() {
        let result = tokenize("False");
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
