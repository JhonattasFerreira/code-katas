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
    match iter.next() {
        Some(Token::String(_)) => {}
        _ => return Err(String::from("expected string value")),
    }
    Ok(())
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
    fn test_invalid_missing_colon() {
        let tokens = vec![
            Token::LeftBrace,
            Token::String(String::from("key")),
            Token::String(String::from("value")),
            Token::RightBrace,
        ];
        assert!(parse(&tokens).is_err());
    }
}
