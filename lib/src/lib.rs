use std::fmt::Formatter;

#[derive(Debug)]
pub struct OrionError {
    message: String,
    line: u32,
    file: String,
}

impl std::fmt::Display for OrionError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Error: {}, {}:{}", self.message, self.file, self.line)?;
        Ok(())
    }
}

pub type Result<T> = std::result::Result<T, OrionError>;
#[macro_export]
macro_rules! error {
    () => {
        OrionError {
            message: "".to_owned(),
            line: line!(),
            file: file!().to_owned(),
        }
    };
    ($($msg:tt),*) => {
        {
            use crate::OrionError;
            let mut message = String::new();
            $(
                message.push_str(&format!("{} ", $msg));
            )*
            message.pop();
            OrionError {
                message,
                line: line!(),
                file: file!().to_owned(),
            }
        }
    }
}

mod lexer;
mod parser;

#[cfg(test)]
mod tests {
    use crate::lexer::lexer::Lexer;
    use crate::lexer::tokens::Token;
    use crate::parser::node::*;
    use crate::parser::parser::Parser;

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
            let mut lexer = Lexer::new("# a looooooooooooooooooong comment\n45".to_owned());
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

    mod parsing {

        use super::*;

        #[test]
        fn multiple_args() -> crate::Result<()> {
            let mut lexer = Lexer::new("(print \"foo\" 4 nil true)".to_owned());
            let mut parser = Parser::new( lexer.scan_tokens());
            let ast = parser.parse_tokens()?;

            assert_eq!(ast, Node {
                ntype: NodeType::Scope,
                children: vec![
                    Node {
                        ntype: NodeType::FunctionCall(
                            "print".to_owned(),
                        ),
                        children: vec![
                            Node {
                                ntype: NodeType::String(
                                    "foo".to_owned(),
                                ),
                                children: vec![],
                            },
                            Node {
                                ntype: NodeType::Int(
                                    4
                                ),
                                children: vec![],
                            },
                            Node {
                                ntype: NodeType::Nil,
                                children: vec![],
                            },
                            Node {
                                ntype: NodeType::Bool(
                                    true
                                ),
                                children: vec![],
                            },
                        ],
                    },
                ],
            });

            Ok(())
        }

    }
}

