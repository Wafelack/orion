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
use crate::{
    bug, error,
    lexer::{TType, Token},
    Result,
};
use std::{collections::HashMap, mem::discriminant};

#[derive(PartialEq, Debug, Clone)]
pub struct Expr {
    pub line: usize,
    pub exprt: ExprT,
}
impl Expr {
    pub fn new(exprt: ExprT) -> Self {
        Self {
            exprt,
            line: 1,
        }
    }
    pub fn line(self, line: usize) -> Self {
        Self {
            exprt: self.exprt,
            line,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprT {
    Var(String),
    Call(Box<Expr>, Vec<Expr>),
    Lambda(Vec<String>, Box<Expr>),
    Literal(Literal),
    Def(String, Box<Expr>, bool), // (name, value, impure?)
    Constr(String, Vec<Expr>),
    Enum(String, HashMap<String, u8>),
    Tuple(Vec<Expr>),
    Load(Vec<String>),
    Match(Box<Expr>, Vec<(Pattern, Expr)>),
    Begin(Vec<Expr>),
    Builtin(String, Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i32),
    Single(f32),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Var(String),
    Constr(String, Vec<Pattern>),
    Tuple(Vec<Pattern>),
    Literal(Literal),
}
fn first_char(s: impl ToString) -> char {
    s.to_string().chars().nth(0).unwrap()
}

pub struct Parser {
    input: Vec<Token>,
    output: Vec<Expr>,
    current: usize,
    file: String,
}

impl Parser {
    pub fn new(input: Vec<Token>, file: impl ToString) -> Self {
        Self {
            input,
            output: vec![],
            current: 0usize,
            file: file.to_string(),
        }
    }
    fn advance(&mut self, expected: TType) -> Result<Token> {
        let popped = self.pop()?;

        if discriminant(&popped.ttype) != discriminant(&expected) {
            error!(
                self.file,
                popped.line =>
                "Expected {}, found {}.",
                expected.get_type(),
                popped.ttype.get_type()
                )
        } else {
            Ok(popped)
        }
    }
    fn pop(&mut self) -> Result<Token> {
        if self.is_at_end() {
            let previous = &self.input[self.current - 1];
            error!(
                self.file,
                previous.line =>
                "Unfinished expression.",
                )
        } else {
            if self.input.len() != 1 {
                self.current += 1;
            }
            Ok(self.input[self.current - if self.input.len() == 1 { 0 } else { 1 }].clone())
        }
    }
    fn peek(&self) -> Option<Token> {
        self.input
            .iter()
            .nth(self.current)
            .and_then(|t| Some(t.clone()))
    }
    fn is_at_end(&self) -> bool {
        self.input.len() != 1 && self.current >= self.input.len()
    }
    fn advance_many(&mut self, expected: TType) -> Result<Vec<Token>> {
        let mut toret = vec![];

        while !self.is_at_end()
            && std::mem::discriminant(&self.peek().unwrap().ttype) == discriminant(&expected)
            {
                toret.push(self.advance(expected.clone())?);
            }

        Ok(toret)
    }

    fn parse_pattern(&mut self) -> Result<Pattern> {
        let root = self.pop()?;

        Ok(match &root.ttype {
            TType::Str(s) => Pattern::Literal(Literal::String(s.to_string())),
            TType::Number(i) => Pattern::Literal(Literal::Integer(*i)),
            TType::Float(f) => Pattern::Literal(Literal::Single(*f)),
            TType::Ident(v) => {
                if first_char(&v).is_ascii_uppercase() {
                    Pattern::Constr(v.to_string(), vec![])
                } else {
                    Pattern::Var(v.to_string())
                }
            }
            TType::LParen => {
                let subroot = self.pop()?;

                match &subroot.ttype {
                    TType::RParen => Pattern::Tuple(vec![]),
                    TType::Tuple => {
                        let mut args = vec![];
                        while !self.is_at_end() && self.peek().unwrap().ttype != TType::RParen {
                            args.push(self.parse_pattern()?);
                        }

                        self.advance(TType::RParen)?;

                        Pattern::Tuple(args)
                    }
                    TType::LParen | TType::Ident(_) => {
                        let mut args = vec![];
                        while !self.is_at_end() && self.peek().unwrap().ttype != TType::RParen {
                            args.push(self.parse_pattern()?);
                        }
                        self.advance(TType::RParen)?;

                        if let TType::Ident(x) = &subroot.ttype {
                            if first_char(x).is_ascii_uppercase() {
                                Pattern::Constr(x.to_string(), args)
                            } else {
                                return error!(
                                    self.file,
                                    subroot.line =>
                                    "Invalid Enum Variant name, Enum Variant names have to start with an uppercase letter: {}.",
                                    x
                                    );
                            }
                        } else {
                            return error!(
                                self.file,
                                subroot.line =>
                                "Expected an Enum Variant.",
                                );
                        }
                    }
                    _ => {
                        return error!(
                            self.file,
                            subroot.line =>
                            "Expected Tuple or Enum Variant, found {}.",
                            subroot.ttype.get_type(),
                            )
                    }
                }
            }
            _ => {
                return error!(
                    self.file,
                    root.line =>
                    "Expected Literal, Identifier, Tuple or Enum Variant, found {}.",
                    root.ttype.get_type()
                    )
            }
        })
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        let root = self.pop()?;

        Ok(match &root.ttype {
            TType::Str(s) => Expr::new(ExprT::Literal(Literal::String(s.to_string()))).line(root.line),
            TType::Float(f) => Expr::new(ExprT::Literal(Literal::Single(*f))).line(root.line),
            TType::Number(i) => Expr::new(ExprT::Literal(Literal::Integer(*i))).line(root.line),
            TType::Ident(v) => {
                if first_char(&v).is_ascii_uppercase() {
                    Expr::new(ExprT::Constr(v.to_string(), vec![])).line(root.line)
                } else {
                    Expr::new(ExprT::Var(v.to_string())).line(root.line)
                }
            }
            TType::Quote => Expr::new(ExprT::Lambda(vec![], Box::new(self.parse_expr()?))).line(root.line),
            TType::LBrace => {
                let mut expressions = vec![];

                while !self.is_at_end() && self.peek().unwrap().ttype != TType::RBrace {
                    expressions.push(self.parse_expr()?);
                }
                self.advance(TType::RBrace)?;
                Expr::new(ExprT::Begin(expressions)).line(root.line)
            }
            TType::LParen => {
                let subroot = self.pop()?;

                match &subroot.ttype {
                    TType::Builtin(b) => {
                        let mut args = vec![];
                        while !self.is_at_end() && self.peek().unwrap().ttype != TType::RParen {
                            args.push(self.parse_expr()?);
                        }
                        self.advance(TType::RParen)?;
                        Expr::new(ExprT::Builtin(b.to_string(), args)).line(subroot.line)
                    }
                    TType::Load => {
                        let mut names = vec![];
                        while !self.is_at_end() && self.peek().unwrap().ttype != TType::RParen {
                            let r_name = self.advance(TType::Str("".to_owned()))?;

                            if let TType::Str(n) = r_name.ttype {
                                names.push(n);
                            } else {
                                bug!("UNEXPECTED_STRING")
                            }
                        }
                        self.advance(TType::RParen)?;
                        Expr::new(ExprT::Load(names)).line(subroot.line)                    
                    }
                    TType::Def => {
                        let impure =
                            if self.peek().and_then(|t| Some(t.ttype)) == Some(TType::Quote) {
                                self.advance(TType::Quote)?;
                                let got = self.advance(TType::Ident("".to_string()))?;
                                if got.ttype == TType::Ident("impure".to_string()) {
                                    true
                                } else {
                                    false
                                }
                            } else {
                                false
                            };
                        let raw_name = self.advance(TType::Ident("".to_owned()))?;
                        let name = if let TType::Ident(n) = raw_name.ttype {
                            n
                        } else {
                            bug!("UNEXPECTED_NON_IDENTIFIER");
                        };
                        if first_char(&name).is_ascii_uppercase() {
                            return error!(
                                self.file,
                                subroot.line =>
                                "Literal names have to start with a lowercase letter.",
                                );
                        }

                        let value = self.parse_expr()?;

                        self.advance(TType::RParen)?;

                        Expr::new(ExprT::Def(name, Box::new(value), impure)).line(subroot.line)
                    }
                    TType::Begin => {
                        let mut expressions = vec![];

                        while !self.is_at_end() && self.peek().unwrap().ttype != TType::RParen {
                            expressions.push(self.parse_expr()?);
                        }

                        self.advance(TType::RParen)?;

                        Expr::new(ExprT::Begin(expressions)).line(subroot.line)
                    }
                    TType::Match => {
                        let to_match = self.parse_expr()?;
                        let mut couples = vec![];
                        while !self.is_at_end() && self.peek().unwrap().ttype != TType::RParen {
                            self.advance(TType::LParen)?;
                            let pat = self.parse_pattern()?;
                            let todo = self.parse_expr()?;
                            couples.push((pat, todo));
                            self.advance(TType::RParen)?;
                        }
                        self.advance(TType::RParen)?;
                        Expr::new(ExprT::Match(Box::new(to_match), couples)).line(subroot.line)
                    }
                    TType::Enum => {
                        let r_name = self.advance(TType::Ident("".to_owned()))?;

                        let name = if let TType::Ident(n) = r_name.ttype {
                            n
                        } else {
                            bug!("UNEXPECTED_NON_IDENTIFIER");
                        };

                        if !first_char(&name).is_ascii_uppercase() {
                            return error!(
                                self.file,
                                r_name.line =>
                                "Enum names have to start with a uppercase letter.",
                                );
                        }

                        let mut var_len = HashMap::new();
                        while !self.is_at_end() && self.peek().unwrap().ttype != TType::RParen {
                            let mul = if self.peek().unwrap().ttype == TType::LParen {
                                self.advance(TType::LParen)?;
                                true
                            } else {
                                false
                            };

                            let r_name = self.advance(TType::Ident("".to_owned()))?;

                            let vname = if let TType::Ident(n) = r_name.ttype {
                                n
                            } else {
                                bug!("UNEXPECTED_NON_IDENTIFIER");
                            };

                            if !first_char(&vname).is_ascii_uppercase() {
                                return error!(self.file, r_name.line => "Enum variant names have to start with a uppercase letter.");
                            }

                            let length = if mul {
                                self.advance_many(TType::Ident("".to_owned()))?.len() as u8
                            } else {
                                0u8
                            };

                            var_len.insert(vname, length);

                            if mul {
                                self.advance(TType::RParen)?;
                            }
                        }

                        self.advance(TType::RParen)?;

                        Expr::new(ExprT::Enum(name, var_len)).line(subroot.line)
                    }
                    TType::Lambda => {
                        self.advance(TType::LParen)?;
                        let args = self.advance_many(TType::Ident("".to_owned()))?;
                        self.advance(TType::RParen)?;

                        let args = args
                            .iter()
                            .map(|e| {
                                if let TType::Ident(ident) = &e.ttype {
                                    ident.to_string()
                                } else {
                                    bug!("UNEXPECTED_NON_IDENTIFIER");
                                }
                            })
                        .collect::<Vec<String>>();

                        let body = self.parse_expr()?;

                        self.advance(TType::RParen)?;
                        Expr::new(ExprT::Lambda(args, Box::new(body))).line(subroot.line)
                    }
                    TType::Tuple => {
                        let mut args = vec![];
                        while !self.is_at_end() && self.peek().unwrap().ttype != TType::RParen {
                            args.push(self.parse_expr()?);
                        }
                        self.advance(TType::RParen)?;
                        Expr::new(ExprT::Tuple(args)).line(subroot.line)
                    }
                    TType::RParen => Expr::new(ExprT::Tuple(vec![])).line(subroot.line),
                    TType::LParen | TType::Ident(_) => {
                        self.current -= 1; // Safe because at least 1 paren
                        let func = self.parse_expr()?;

                        let mut args = vec![];
                        while !self.is_at_end() && self.peek().unwrap().ttype != TType::RParen {
                            args.push(self.parse_expr()?);
                        }

                        self.advance(TType::RParen)?;

                        if let TType::Ident(x) = &subroot.ttype {
                            if first_char(x).is_ascii_uppercase() {
                                Expr::new(ExprT::Constr(x.to_string(), args)).line(subroot.line)
                            } else {
                                Expr::new(ExprT::Call(Box::new(func), args)).line(subroot.line)
                            }
                        } else {
                            Expr::new(ExprT::Call(Box::new(func), args)).line(subroot.line)
                        }
                    }
                    _ => return error!(self.file, subroot.line => "Unexpected Literal."),
                }
            }
            TType::RParen => {
                return error!(
                    self.file,
                    root.line => 
                    "Unexpected Closing Parenthese.",
                    )
            }
            _ => return error!(self.file, root.line => "Unexpected Keyword."),
        })
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>> {
        while !self.is_at_end() {
            let to_push = self.parse_expr()?;
            self.output.push(to_push);

            if self.input.len() == 1 {
                break;
            }
        }

        Ok(self.output.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{lexer::Lexer, table};

    #[test]
    fn variables() -> Result<()> {
        let tokens = Lexer::new("foo", 0).proc_tokens()?;
        let ast = Parser::new(tokens, "TEST").parse()?;

        assert_eq!(ast, vec![Expr::new(ExprT::Var("foo".to_string())).line(1)]);

        Ok(())
    }

    #[test]
    fn call() -> Result<()> {
        let tokens = Lexer::new("(foobar 4 5)", 0).proc_tokens()?;
        let ast = Parser::new(tokens, "TEST").parse()?;

        assert_eq!(
            ast,
            vec![Expr::new(ExprT::Call(
                    Box::new(Expr::new(ExprT::Var("foobar".to_string())).line(1)),
                    vec![
                    Expr::new(ExprT::Literal(Literal::Integer(4))).line(1),
                    Expr::new(ExprT::Literal(Literal::Integer(5))).line(1)
                    ]
                    )).line(1)]
            );

        Ok(())
    }

    #[test]
    fn literal() -> Result<()> {
        let tokens = Lexer::new("\"foo\" 42 3.1415926535897932", 0).proc_tokens()?;
        let ast = Parser::new(tokens, "TEST").parse()?;

        assert_eq!(
            ast,
            vec![
            Expr::new(ExprT::Literal(Literal::String("foo".to_string()))).line(1),
            Expr::new(ExprT::Literal(Literal::Integer(42))).line(1),
            Expr::new(ExprT::Literal(Literal::Single(3.1415926535897932))).line(1),
            ]
            );

        Ok(())
    }

    #[test]
    fn lambda() -> Result<()> {
        let tokens = Lexer::new("(λ (x y) 5)", 0).proc_tokens()?;
        let ast = Parser::new(tokens, "TEST").parse()?;

        assert_eq!(
            ast,
            vec![Expr::new(ExprT::Lambda(
                    vec!["x".to_string(), "y".to_string()],
                    Box::new(Expr::new(ExprT::Literal(Literal::Integer(5))).line(1))
                    )).line(1)]
            );

        Ok(())
    }

    #[test]
    fn def() -> Result<()> {
        let tokens = Lexer::new("(def foo 5)(def 'impure moo jsp)", 0).proc_tokens()?;
        let ast = Parser::new(tokens, "TEST").parse()?;

        assert_eq!(
            ast,
            vec![
            Expr::new(ExprT::Def(
                    "foo".to_string(),
                    Box::new(Expr::new(ExprT::Literal(Literal::Integer(5)))),
                    false)),
                    Expr::new(ExprT::Def(
                            "moo".to_string(),
                            Box::new(Expr::new(ExprT::Var("jsp".to_string()))),
                            true))]);

        Ok(())
    }

    #[test]
    fn constr() -> Result<()> {
        let tokens = Lexer::new("(Just a)Nothing", 0).proc_tokens()?;
        let ast = Parser::new(tokens, "TEST").parse()?;

        assert_eq!(
            ast,
            vec![
            Expr::new(ExprT::Constr("Just".to_string(), vec![Expr::new(ExprT::Var("a".to_string()))])),
            Expr::new(ExprT::Constr("Nothing".to_string(), vec![]))]);

        Ok(())
    }

    #[test]
    fn r#enum() -> Result<()> {
        let tokens = Lexer::new("(enum Maybe (Just x) Nil)", 0).proc_tokens()?;
        let ast = Parser::new(tokens, "TEST").parse()?;

        assert_eq!(
            ast,
            vec![Expr::new(ExprT::Enum(
                    "Maybe".to_string(),
                    table! {"Just".to_string() => 1u8, "Nil".to_string() => 0u8}
                    ))]
            );

        Ok(())
    }

    #[test]
    fn tuple() -> Result<()> {
        let tokens = Lexer::new("(, a b c)", 0).proc_tokens()?;
        let ast = Parser::new(tokens, "TEST").parse()?;

        assert_eq!(
            ast,
            vec![Expr::new(ExprT::Tuple(vec![
                                        Expr::new(ExprT::Var("a".to_string())),
                                        Expr::new(ExprT::Var("b".to_string())),
                                        Expr::new(ExprT::Var("c".to_string()))]))]);
        Ok(())
    }

    #[test]
    fn load() -> Result<()> {
        let tokens = Lexer::new("(load \"foo\" \"bar\")", 0).proc_tokens()?;
        let ast = Parser::new(tokens, "TEST").parse()?;

        assert_eq!(
            ast,
            vec![Expr::new(ExprT::Load(vec!["foo".to_string(), "bar".to_string()]))]
            );

        Ok(())
    }

    #[test]
    fn r#match() -> Result<()> {
        let tokens = Lexer::new("(match foo (bar x)(_ 9))", 0).proc_tokens()?;
        let ast = Parser::new(tokens, "TEST").parse()?;

        assert_eq!(
            ast,
            vec![Expr::new(ExprT::Match(
                    Box::new(Expr::new(ExprT::Var("foo".to_string()))),
                    vec![
                    (Pattern::Var("bar".to_string()), Expr::new(ExprT::Var("x".to_string()))),
                    (Pattern::Var("_".to_string()),
                    Expr::new(ExprT::Literal(Literal::Integer(9))))]))]);

        Ok(())
    }

    #[test]
    fn begin() -> Result<()> {
        let tokens = Lexer::new("(begin a b c) { }", 0).proc_tokens()?;
        let ast = Parser::new(tokens, "TEST").parse()?;
        assert_eq!(
            ast,
            vec![Expr::new(ExprT::Begin(vec![
                                        Expr::new(ExprT::Var("a".to_string())),
                                        Expr::new(ExprT::Var("b".to_string())),
                                        Expr::new(ExprT::Var("c".to_string()))])), Expr::new(ExprT::Begin(vec![]))]);
        Ok(())
    }

    #[test]
    fn quote() -> Result<()> {
        let tokens = Lexer::new("'a", 0).proc_tokens()?;
        let ast = Parser::new(tokens, "TEST").parse()?;
        assert_eq!(ast, vec![Expr::new(ExprT::Lambda(vec![], Box::new(Expr::new(ExprT::Var("a".to_string())))))]);
        Ok(())
    }

    #[test]
    fn builtins() -> Result<()> {
        let tokens = Lexer::new("(format 5 a)", 0).proc_tokens()?;
        let ast = Parser::new(tokens, "TEST").parse()?;
        assert_eq!(
            ast,
            vec![Expr::new(ExprT::Builtin(
                    "format".to_string(),
                    vec![
                    Expr::new(ExprT::Literal(Literal::Integer(5))),
                    Expr::new(ExprT::Var("a".to_string()))]))]);
        Ok(())
    }
}
