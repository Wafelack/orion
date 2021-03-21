#[cfg(test)]
mod test {
    use crate::{Result, lexer::{Lexer, TType, Token}};

#[test]
    fn lexing() -> Result<()> {
        let mut lexer = Lexer::new("()\"Foo Bar\"True False TrueFalse 4 9.1");
        let tokens = lexer.proc_tokens()?;

        assert_eq!(tokens,
                   vec![Token::new(TType::LParen, 1, 1), Token::new(TType::RParen, 2, 1), Token::new(TType::Str("Foo Bar".to_owned()), 11, 1), Token::new(TType::Bool(true), 15, 1), Token::new(TType::Bool(false), 21, 1), Token::new(TType::Ident("TrueFalse".to_owned()), 31, 1), Token::new(TType::Number(4), 33, 1), Token::new(TType::Float(9.1), 37, 1)]);

        Ok(())
    }

}
