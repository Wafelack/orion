use std::fmt::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    Identifier(String),
    Nil,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {

        match self {
            Token::String(s) => write!(f, "\"{}\"", s)?,
            Token::Int(i) => write!(f, "{}", i)?,
            Token::Float(s) => write!(f, "{}", s)?,
            Token::Bool(b) => write!(f, "{}", b)?,
            Token::Identifier(s) => write!(f, "{}", s)?,
            Token::LeftParen => write!(f, "(")?,
            Token::RightParen => write!(f,")")?,
            Token::LeftBrace => write!(f, "{{")?,
            Token::RightBrace => write!(f, "}}")?,
            Token::Nil => write!(f, "nil")?,
        }
        Ok(())
    }
}