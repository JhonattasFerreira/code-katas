use crate::lexer::Token;
use std::iter::Peekable;
use std::slice::Iter;

pub fn parse(tokens: &[Token]) -> Result<(), String> {
    let mut iter = tokens.iter().peekable();
    parse_object(&mut iter)?;
    if iter.next().is_some() {
        return Err(String::from("unexpected tokens after end of object"));
    }
    Ok(())
}

fn parse_object(iter: &mut Peekable<Iter<Token>>) -> Result<(), String> {
    match iter.next() {
        Some(Token::LeftBrace) => {}
        _ => return Err(String::from("expected '{'")),
    }

    if let Some(Token::RightBrace) = iter.peek() {
        iter.next();
        return Ok(());
    }

    parse_key_value(iter)?;

    loop {
        match iter.next() {
            Some(Token::RightBrace) => return Ok(()),
            Some(Token::Comma) => {
                if let Some(Token::RightBrace) = iter.peek() {
                    return Err(String::from("trailing comma"));
                }
                parse_key_value(iter)?;
            }
            _ => return Err(String::from("expected ',' or '}'")),
        }
    }
}

fn parse_key_value(iter: &mut Peekable<Iter<Token>>) -> Result<(), String> {
    match iter.next() {
        Some(Token::String(_)) => {}
        _ => return Err(String::from("expected string key")),
    }
    match iter.next() {
        Some(Token::Colon) => {}
        _ => return Err(String::from("expected ':'")),
    }
    parse_value(iter)
}

fn parse_value(iter: &mut Peekable<Iter<Token>>) -> Result<(), String> {
    match iter.peek() {
        Some(Token::LeftBrace) => parse_object(iter),
        Some(Token::LeftBracket) => parse_array(iter),
        _ => match iter.next() {
            Some(Token::String(_)) => Ok(()),
            Some(Token::Bool(_)) => Ok(()),
            Some(Token::Null) => Ok(()),
            Some(Token::Number(_)) => Ok(()),
            _ => Err(String::from("expected a value")),
        },
    }
}

fn parse_array(iter: &mut Peekable<Iter<Token>>) -> Result<(), String> {
    match iter.next() {
        Some(Token::LeftBracket) => {}
        _ => return Err(String::from("expected '['")),
    }

    if let Some(Token::RightBracket) = iter.peek() {
        iter.next();
        return Ok(());
    }

    parse_value(iter)?;

    loop {
        match iter.next() {
            Some(Token::RightBracket) => return Ok(()),
            Some(Token::Comma) => {
                if let Some(Token::RightBracket) = iter.peek() {
                    return Err(String::from("trailing comma in array"));
                }
                parse_value(iter)?;
            }
            _ => return Err(String::from("expected ',' or ']'")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Token;

    #[test]
    fn test_valid_empty_object() {
        let tokens = vec![Token::LeftBrace, Token::RightBrace];
        assert_eq!(parse(&tokens), Ok(()));
    }

    #[test]
    fn test_empty_tokens() {
        let tokens = vec![];
        assert!(parse(&tokens).is_err());
    }

    #[test]
    fn test_only_left_brace() {
        let tokens = vec![Token::LeftBrace];
        assert!(parse(&tokens).is_err());
    }

    #[test]
    fn test_only_right_brace() {
        let tokens = vec![Token::RightBrace];
        assert!(parse(&tokens).is_err());
    }

    #[test]
    fn test_valid_single_key_value() {
        let tokens = vec![
            Token::LeftBrace,
            Token::String(String::from("key")),
            Token::Colon,
            Token::String(String::from("value")),
            Token::RightBrace,
        ];
        assert_eq!(parse(&tokens), Ok(()));
    }

    #[test]
    fn test_valid_multiple_key_values() {
        let tokens = vec![
            Token::LeftBrace,
            Token::String(String::from("key")),
            Token::Colon,
            Token::String(String::from("value")),
            Token::Comma,
            Token::String(String::from("key2")),
            Token::Colon,
            Token::String(String::from("value")),
            Token::RightBrace,
        ];
        assert_eq!(parse(&tokens), Ok(()));
    }

    #[test]
    fn test_invalid_trailing_comma() {
        let tokens = vec![
            Token::LeftBrace,
            Token::String(String::from("key")),
            Token::Colon,
            Token::String(String::from("value")),
            Token::Comma,
            Token::RightBrace,
        ];
        assert!(parse(&tokens).is_err());
    }

    #[test]
    fn test_valid_bool_true_value() {
        let tokens = vec![
            Token::LeftBrace,
            Token::String(String::from("key")),
            Token::Colon,
            Token::Bool(true),
            Token::RightBrace,
        ];
        assert_eq!(parse(&tokens), Ok(()));
    }

    #[test]
    fn test_valid_bool_false_value() {
        let tokens = vec![
            Token::LeftBrace,
            Token::String(String::from("key")),
            Token::Colon,
            Token::Bool(false),
            Token::RightBrace,
        ];
        assert_eq!(parse(&tokens), Ok(()));
    }

    #[test]
    fn test_valid_null_value() {
        let tokens = vec![
            Token::LeftBrace,
            Token::String(String::from("key")),
            Token::Colon,
            Token::Null,
            Token::RightBrace,
        ];
        assert_eq!(parse(&tokens), Ok(()));
    }

    #[test]
    fn test_valid_number_value() {
        let tokens = vec![
            Token::LeftBrace,
            Token::String(String::from("key")),
            Token::Colon,
            Token::Number(101.0),
            Token::RightBrace,
        ];
        assert_eq!(parse(&tokens), Ok(()));
    }

    #[test]
    fn test_invalid_missing_colon() {
        let tokens = vec![
            Token::LeftBrace,
            Token::String(String::from("key")),
            Token::String(String::from("value")),
            Token::RightBrace,
        ];
        assert!(parse(&tokens).is_err());
    }

    #[test]
    fn test_valid_empty_array_value() {
        let tokens = vec![
            Token::LeftBrace,
            Token::String(String::from("key")),
            Token::Colon,
            Token::LeftBracket,
            Token::RightBracket,
            Token::RightBrace,
        ];
        assert_eq!(parse(&tokens), Ok(()));
    }

    #[test]
    fn test_valid_array_with_values() {
        let tokens = vec![
            Token::LeftBrace,
            Token::String(String::from("key")),
            Token::Colon,
            Token::LeftBracket,
            Token::String(String::from("value")),
            Token::Comma,
            Token::Number(1.0),
            Token::RightBracket,
            Token::RightBrace,
        ];
        assert_eq!(parse(&tokens), Ok(()));
    }

    #[test]
    fn test_valid_nested_object_value() {
        let tokens = vec![
            Token::LeftBrace,
            Token::String(String::from("key")),
            Token::Colon,
            Token::LeftBrace,
            Token::RightBrace,
            Token::RightBrace,
        ];
        assert_eq!(parse(&tokens), Ok(()));
    }

    #[test]
    fn test_invalid_array_trailing_comma() {
        let tokens = vec![
            Token::LeftBrace,
            Token::String(String::from("key")),
            Token::Colon,
            Token::LeftBracket,
            Token::String(String::from("value")),
            Token::Comma,
            Token::RightBracket,
            Token::RightBrace,
        ];
        assert!(parse(&tokens).is_err());
    }

    #[test]
    fn test_invalid_unclosed_object_after_array() {
        let tokens = vec![
            Token::LeftBrace,
            Token::String(String::from("key")),
            Token::Colon,
            Token::LeftBracket,
            Token::RightBracket,
        ];
        assert!(parse(&tokens).is_err());
    }
}
