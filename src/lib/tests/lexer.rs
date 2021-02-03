#[cfg(test)]
mod test {

    use crate::lexer::lexer::Lexer;
    use crate::lexer::tokens::Token;

    mod tokenizing {
        use super::*;

        #[test]
        fn string() {
            let mut lexer = Lexer::new("\"foo\"".to_owned());
            assert_eq!(lexer.scan_tokens(), vec![Token::String("foo".to_owned())]);
        }

        #[test]
        fn float() {
            let mut lexer = Lexer::new("43.50".to_owned());
            assert_eq!(lexer.scan_tokens(), vec![Token::Float(43.50)]);
        }

        #[test]
        fn int() {
            let mut lexer = Lexer::new("43".to_owned());
            assert_eq!(lexer.scan_tokens(), vec![Token::Int(43)]);
        }

        #[test]
        fn boolean() {
            let mut lexer = Lexer::new("true false".to_owned());
            assert_eq!(lexer.scan_tokens(), vec![Token::Bool(true), Token::Bool(false)]);
        }

        #[test]
        fn nil() {
            let mut lexer = Lexer::new("n ni nil".to_owned());
            assert_eq!(lexer.scan_tokens(), vec![Token::Identifier("n".to_owned()), Token::Identifier("ni".to_owned()), Token::Nil]);
        }

        #[test]
        fn comment() {
            let mut lexer = Lexer::new("; a looooooooooooooooooong comment\n45".to_owned());
            assert_eq!(lexer.scan_tokens(), vec![Token::Int(45)]);
        }

        #[test]
        fn paren() {
            let mut lexer = Lexer::new("()".to_owned());
            assert_eq!(lexer.scan_tokens(), vec![Token::LeftParen, Token::RightParen]);
        }

        #[test]
        fn brace() {
            let mut lexer = Lexer::new("{}".to_owned());
            assert_eq!(lexer.scan_tokens(), vec![Token::LeftBrace, Token::RightBrace]);
        }

        #[test]
        fn identifier() {
            let mut lexer = Lexer::new("def".to_owned());
            assert_eq!(lexer.scan_tokens(), vec![Token::Identifier("def".to_owned())]);
        }
    }
}