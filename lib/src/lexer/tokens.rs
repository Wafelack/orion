pub enum Token {
    String(String),
    Int(i32),
    Float(f32),
    Identifier(String),

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
}