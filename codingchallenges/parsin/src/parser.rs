use crate::lexer::Token;

pub fn parse(tokens: &[Token]) -> Result<(), String> {
    match tokens {
        [Token::LeftBrace, Token::RightBrace] => Ok(()),
        [] => Err(String::from("empty input")),
        _ => Err(String::from("invalid JSON")),
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
}
