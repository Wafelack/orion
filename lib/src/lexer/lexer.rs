use crate::lexer::tokens::Token;

pub struct Lexer {
    code: String,
    tokens: Vec<Token>,
    current: usize,
    start: usize,
    line: usize,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Self {
            code,
            tokens: vec![],
            current: 0,
            start: 0,
            line: 0,
        }
    }
    fn add_token(&mut self, tok: Token) {
        self.tokens.push(tok);
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.code.len()
    }
    fn advance(&mut self) -> char {
        self.current += 1;
        self.code.chars().nth(self.current - 1).unwrap_or('\0')
    }
    fn peek(&mut self) -> char {
        self.code.chars().nth(self.current).unwrap_or('\0')
    }
    fn peek_next(&mut self) -> char {
        self.code.chars().nth(self.current + 1).unwrap_or('\0')
    }
    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(Token::LeftParen),
            ')' => self.add_token(Token::RightParen),
            '{' => self.add_token(Token::LeftBrace),
            '}' => self.add_token(Token::RightBrace),
            '"' => self.string(),
            '#' => while !self.is_at_end() && self.peek() != '\n' {
                self.advance();
            }
            'n' => if self.peek() == 'i' {
                    if self.peek_next() == 'l' {
                        self.advance();
                        self.advance();
                        self.add_token(Token::Nil);
                    } else {
                        self.identifier();
                    }
                } else {    
                    self.identifier();
                }
            ' ' | '\t' | '\r' => {},
            '\n' => self.line += 1,
            '-' => if self.peek().is_digit(10) {
                self.advance();
                self.number();
            } else {
                self.identifier();
            }
            x => if x.is_digit(10) {
                self.number();
            } else {
                self.identifier();
            }
        }
    }
    fn number(&mut self) {
        while self.peek().is_digit(10) && !self.is_at_end() {
            self.advance();
        }
        if self.peek() == '.' {
            self.advance();
        }
        while self.peek().is_digit(10) && !self.is_at_end() {
            self.advance();
        }

        let raw = &self.code[self.start..self.current];
        match raw.parse::<i32>() {
            Ok(i) => self.add_token(Token::Int(i)),
            Err(_) => {
                match raw.parse::<f32>() {
                    Ok(f) => self.add_token(Token::Float(f)),
                    _ => {}
                }
            }
        }

    }
    fn identifier(&mut self) {
        let is_whitespace_or_end = |c: char| -> bool {
            c == ' ' || c == '\r' || c == '\n' || c == '\t' || c == ')' || c == '}' || c == '(' || c == '{'
        };

        while !is_whitespace_or_end(self.peek()) && !self.is_at_end() {
            self.advance();
        }

        let raw = &self.code[self.start..self.current];
        if raw == "true" {
            self.add_token(Token::Bool(true));
        } else if raw == "false" {
            self.add_token(Token::Bool(false));
        } else {
            self.add_token(Token::Identifier(raw.to_owned()));
        }
    }
    fn string(&mut self) {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        self.advance(); // closing delimiter

        let toret = (&self.code[self.start + 1..self.current - 1]).to_owned();

        self.tokens.push(
            Token::String(toret)
        );
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.clone()
    }

}