use crate::{error, OrionError, Result};

#[derive(Clone, PartialEq, Debug)]
pub enum TType {
    LParen,
    RParen,
    Str(String),
    Number(i32),
    Float(f32),
    Ident(String),
    Def,
    Enum,
    Tuple,
    Lambda,
}

impl TType {
    pub fn get_type(&self) -> String {
        match self {
            Self::LParen => "Opening Parenthese",
            Self::RParen => "Closing Parenthese",
            Self::Str(_) => "String",
            Self::Number(_) => "Integer",
            Self::Float(_) => "Float",
            Self::Ident(_) => "Identifier",
            _ => "Keyword",
        }
        .to_string()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub line: usize,
    pub ttype: TType,
    pub col: usize,
}

impl Token {
    pub fn new(ttype: TType, col: usize, line: usize) -> Self {
        Self { line, ttype, col }
    }
}

pub struct Lexer {
    input: String,
    output: Vec<Token>,
    current: usize,
    line: usize,
    start: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: impl ToString) -> Self {
        Self {
            input: input.to_string().replace("λ", "lambda").to_string(),
            output: vec![],
            current: 0,
            column: 0,
            line: 1,
            start: 0,
        }
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.input.chars().collect::<Vec<_>>().len()
    }
    fn peek(&self) -> char {
        self.input.chars().nth(self.current).unwrap()
    }
    fn advance(&mut self) -> char {
        self.column += 1;
        self.current += 1;
        self.input.chars().nth(self.current - 1).unwrap()
    }
    fn add_token(&mut self, ttype: TType) {
        self.output.push(Token::new(ttype, self.column, self.line));
    }
    fn string(&mut self) -> Result<()> {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 0;
            }

            self.advance();
        }

        if self.is_at_end() {
            return error!("{}:{} | Unterminated string.", self.line, self.column);
        }

        self.advance(); // Closing "

        self.add_token(TType::Str(
            self.input[self.start + 1..self.current - 1].to_string(),
        ));

        Ok(())
    }
    fn proc_token(&mut self) -> Result<()> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TType::LParen),
            ')' => self.add_token(TType::RParen),
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.column = 0;
                self.line += 1;
            }
            '"' => self.string()?,
            ';' => {
                while !self.is_at_end() && self.peek() != '\n' {
                    self.advance();
                }
            }
            _ => {
                if c.is_digit(10) {
                    self.number();
                } else {
                    self.identifier();
                }
            }
        }

        Ok(())
    }
    fn number(&mut self) {
        while !self.is_at_end() && self.peek().is_digit(10) {
            self.advance();
        }

        if !self.is_at_end() && self.peek() == '.' {
            self.advance();
        }

        while !self.is_at_end() && self.peek().is_digit(10) {
            self.advance();
        }

        let raw = self.input[self.start..self.current].to_string();

        match raw.parse::<i32>() {
            Ok(i) => self.add_token(TType::Number(i)),
            Err(_) => self.add_token(TType::Float(raw.parse::<f32>().unwrap())),
        }
    }
    fn identifier(&mut self) {
        let stop = vec!['(', ')', ' ', '\t', '\n', '\r'];

        while !self.is_at_end() && !stop.contains(&self.peek()) {
            self.advance();
        }

        let raw = self.input[self.start..self.current].to_string();

        match raw.as_str() {
            "def" => self.add_token(TType::Def),
            "enum" => self.add_token(TType::Enum),
            "lambda" => self.add_token(TType::Lambda),
            "," => self.add_token(TType::Tuple),
            _ => self.add_token(TType::Ident(raw)),
        }
    }
    pub fn proc_tokens(&mut self) -> Result<Vec<Token>> {
        while !self.is_at_end() {
            self.proc_token()?;
            self.start = self.current;
        }

        Ok(self.output.clone())
    }
}
