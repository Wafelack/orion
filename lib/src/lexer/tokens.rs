#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    Identifier(String),

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
}