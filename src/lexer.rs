/*
 *  Copyright (C) 2021, Wafelack <wafelack@protonmail.com>
 *
 *  ------------------------------------------------------
 *
 *     This file is part of Orion.
 *
 *  Orion is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Orion is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Orion.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::{error, OrionError, Result};

#[derive(Clone, PartialEq, Debug)]
pub enum TType {
    LParen,
    RParen,
    Str(String),
    Number(i32),
    Float(f32),
    Ident(String),
    Quote,
    Def,
    Enum,
    Tuple,
    Lambda,
    Match,
    Load,
    Panic,
    Begin,

    Builtin(String),
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
    pub fn display(&self) -> String {
        let ttyped = self.ttype.get_type().to_ascii_lowercase().replace(" ", "_");

        format!("<#token:{}{{{}:{}}}>", ttyped, self.line, self.col)
    }
}

pub struct Lexer {
    input: String,
    output: Vec<Token>,
    current: usize,
    line: usize,
    start: usize,
    column: usize,
    builtins: Vec<String>
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
            builtins: vec![],
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
                apply_ansi_codes(&self.input[self.start + 1..self.current - 1])
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
            '\'' => self.add_token(TType::Quote),
            '"' => self.string()?,
            '#' => if !self.is_at_end() && self.peek() == '!' && self.line == 1 {
                while !self.is_at_end() && self.peek() != '\n' {
                    self.advance();
                }
            } else {
                self.identifier();
            }
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
    fn register_builtin(&mut self, builtin: impl ToString) {
        self.builtins.push(builtin.to_string());
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

        if self.builtins.contains(&raw) {
            self.add_token(TType::Builtin(raw));
        } else {
            match raw.as_str() {
                "def" => self.add_token(TType::Def),
                "enum" => self.add_token(TType::Enum),
                "lambda" => self.add_token(TType::Lambda),
                "," => self.add_token(TType::Tuple),
                "match" => self.add_token(TType::Match),
                "load" => self.add_token(TType::Load),
                "panic" => self.add_token(TType::Panic),
                "begin" => self.add_token(TType::Begin),

                _ => self.add_token(TType::Ident(raw)),
            }

        }

    }
    pub fn proc_tokens(&mut self) -> Result<Vec<Token>> {

        self.register_builtin("format");

        self.register_builtin("putStr");
        self.register_builtin("putStrLn");
        self.register_builtin("write");
        self.register_builtin("getLine");

        self.register_builtin("+");
        self.register_builtin("-");
        self.register_builtin("!");
        self.register_builtin("/");
        self.register_builtin("*");
        self.register_builtin("_cmp");
        self.register_builtin("cos");
        self.register_builtin("sin");
        self.register_builtin("tan");
        self.register_builtin("acos");
        self.register_builtin("asin");
        self.register_builtin("atan");

        self.register_builtin("unquote");

        while !self.is_at_end() {
            self.proc_token()?;
            self.start = self.current;
        }

        Ok(self.output.clone())
    }
}

fn apply_ansi_codes(input: &str) -> String {
    input
        .replace("\\x1b", "\x1b")
        .replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\t", "\t")
        .replace("\\0", "\0")
        .replace("\\\\", "\\")
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_ttypes(input: Vec<Token>) -> Vec<TType> {
        input.into_iter().map(|t| t.ttype).collect::<Vec<_>>()
    }

    #[test]
    fn parentheses() -> Result<()> {
        let ttypes = get_ttypes(Lexer::new("()").proc_tokens()?);
        assert_eq!(ttypes, vec![TType::LParen, TType::RParen]);
        Ok(())
    }

    #[test]
    fn quote() -> Result<()> {
        let ttypes = get_ttypes(Lexer::new("'").proc_tokens()?);
        assert_eq!(ttypes, vec![TType::Quote]);
        Ok(())
    }

    #[test]
    fn numbers() -> Result<()> {
        let ttypes = get_ttypes(Lexer::new("42 3.1415926535897932").proc_tokens()?);
        assert_eq!(ttypes, vec![TType::Number(42), TType::Float(3.1415926535897932)]);
        Ok(())
    }

    #[test]
    fn string() -> Result<()> {
        let ttypes = get_ttypes(Lexer::new(r#""Hello, World !""#).proc_tokens()?);
        assert_eq!(ttypes, vec![TType::Str("Hello, World !".to_string())]);
        Ok(())
    }

    #[test]
    fn def() -> Result<()> {
        let ttypes = get_ttypes(Lexer::new("def").proc_tokens()?);
        assert_eq!(ttypes, vec![TType::Def]);
        Ok(())
    }

    #[test]
    fn r#enum() -> Result<()> {
        let ttypes = get_ttypes(Lexer::new("enum").proc_tokens()?);
        assert_eq!(ttypes, vec![TType::Enum]);
        Ok(())
    }

    #[test]
    fn tuple() -> Result<()> {
        let ttypes = get_ttypes(Lexer::new(",").proc_tokens()?);
        assert_eq!(ttypes, vec![TType::Tuple]);
        Ok(())
    }

    #[test]
    fn lambda() -> Result<()> {
        let ttypes = get_ttypes(Lexer::new("λ lambda").proc_tokens()?);
        assert_eq!(ttypes, vec![TType::Lambda, TType::Lambda]);
        Ok(())
    }

    #[test]
    fn r#match() -> Result<()> {
        let ttypes = get_ttypes(Lexer::new("match").proc_tokens()?);
        assert_eq!(ttypes, vec![TType::Match]);
        Ok(())
    }

    #[test]
    fn panic() -> Result<()> {
        let ttypes = get_ttypes(Lexer::new("panic").proc_tokens()?);
        assert_eq!(ttypes, vec![TType::Panic]);
        Ok(())
    }

    #[test]
    fn begin() -> Result<()> {
        let ttypes = get_ttypes(Lexer::new("begin").proc_tokens()?);
        assert_eq!(ttypes, vec![TType::Begin]);
        Ok(())
    }

    #[test]
    fn load() -> Result<()> {
        let ttypes = get_ttypes(Lexer::new("load").proc_tokens()?);
        assert_eq!(ttypes, vec![TType::Load]);
        Ok(())
    }

    #[test]
    fn builtin() -> Result<()> {
        let ttypes = get_ttypes(Lexer::new("format").proc_tokens()?);
        assert_eq!(ttypes, vec![TType::Builtin("format".to_string())]);
        Ok(())
    }
}
