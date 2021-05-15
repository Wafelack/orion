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
    OrionError, Result,
};
use std::{collections::HashMap, mem::discriminant};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
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
    Panic(String, Box<Expr>),
    Begin(Vec<Expr>),
    Builtin(String, Vec<Expr>),
}

impl Expr {
    pub fn get_type(&self) -> String {
        format!(
            "<#expr:{}>",
            match self {
                Self::Var(s) => format!("constant({})", s),
                Self::Call(_, _) => "call".to_string(),
                Self::Lambda(args, _) => format!("lambda({} args)", args.len()),
                Self::Literal(l) => {
                    match l {
                        Literal::Integer(i) => format!("integer {}", i),
                        Literal::Single(s) => format!("single {}", s),
                        Literal::String(s) => format!("string \"{}\"", s),
                        Literal::Unit => "unit".to_string(),
                    }
                }
                Self::Def(name, _, _) => format!("constant_definition({})", name),
                Self::Constr(name, _) => format!("enum_constructor({})", name),
                Self::Enum(name, _) => format!("enum_definition({})", name),
                Self::Tuple(t) => format!("tuple{{{}}}", t.len()),
                Self::Load(_) => "load_procedure".to_string(),
                Self::Match(_, _) => "match_statement".to_string(),
                Self::Panic(_, _) => "panic_statement".to_string(),
                Self::Begin(_) => "block".to_string(),
                Self::Builtin(n, _) => format!("builtin({})", n),
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i32),
    Single(f32),
    String(String),
    Unit,
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
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Self {
        Self {
            input,
            output: vec![],
            current: 0usize,
        }
    }
    fn advance(&mut self, expected: TType) -> Result<Token> {
        let popped = self.pop()?;

        if discriminant(&popped.ttype) != discriminant(&expected) {
            error!(
                "{}:{} | Expected {}, found {}.",
                popped.line,
                popped.col,
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
                "{}:{} | Unfinished expression.",
                previous.line, previous.col
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
                    TType::RParen => Pattern::Literal(Literal::Unit),
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
                                    "{}:{} | Invalid Enum Variant name, Enum Variant names have to start with an uppercase letter: {}.",
                                    subroot.line, subroot.col, x
                                    );
                            }
                        } else {
                            return error!(
                                "{}:{} | Expected an Enum Variant.",
                                subroot.line, subroot.col
                            );
                        }
                    }
                    _ => {
                        return error!(
                            "{}:{} | Expected Tuple or Enum Variant, found {}.",
                            subroot.line,
                            subroot.col,
                            subroot.ttype.get_type()
                        )
                    }
                }
            }
            _ => {
                return error!(
                    "{}:{} | Expected Literal, Identifier, Tuple or Enum Variant, found {}.",
                    root.line,
                    root.col,
                    root.ttype.get_type()
                )
            }
        })
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        let root = self.pop()?;

        Ok(match &root.ttype {
            TType::Str(s) => Expr::Literal(Literal::String(s.to_string())),
            TType::Float(f) => Expr::Literal(Literal::Single(*f)),
            TType::Number(i) => Expr::Literal(Literal::Integer(*i)),
            TType::Ident(v) => {
                if first_char(&v).is_ascii_uppercase() {
                    Expr::Constr(v.to_string(), vec![])
                } else {
                    Expr::Var(v.to_string())
                }
            }
            TType::Quote => Expr::Lambda(vec![], Box::new(self.parse_expr()?)),
            TType::LParen => {
                let subroot = self.pop()?;

                match &subroot.ttype {
                    TType::Panic => {
                        let to_ret = self.parse_expr()?;
                        self.advance(TType::RParen)?;
                        Expr::Panic(
                            format!("[{}:{}] Program panicked at: ", subroot.line, subroot.col),
                            Box::new(to_ret),
                        )
                    }
                    TType::Builtin(b) => {
                        let mut args = vec![];

                        while !self.is_at_end() && self.peek().unwrap().ttype != TType::RParen {
                            args.push(self.parse_expr()?);
                        }

                        self.advance(TType::RParen)?;

                        Expr::Builtin(b.to_string(), args)
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
                        Expr::Load(names)
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
                                "{}:{} | Literal names have to start with a lowercase letter.",
                                raw_name.line, raw_name.col
                            );
                        }

                        let value = self.parse_expr()?;

                        self.advance(TType::RParen)?;

                        Expr::Def(name, Box::new(value), impure)
                    }
                    TType::Begin => {
                        let mut expressions = vec![];

                        while !self.is_at_end() && self.peek().unwrap().ttype != TType::RParen {
                            expressions.push(self.parse_expr()?);
                        }

                        self.advance(TType::RParen)?;

                        Expr::Begin(expressions)
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

                        Expr::Match(Box::new(to_match), couples)
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
                                "{}:{} | Enum names have to start with a uppercase letter.",
                                r_name.line, r_name.col
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
                                return error!("{}:{} | Enum variant names have to start with a uppercase letter.", r_name.line, r_name.col);
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

                        Expr::Enum(name, var_len)
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
                        Expr::Lambda(args, Box::new(body))
                    }
                    TType::Tuple => {
                        let mut args = vec![];
                        while !self.is_at_end() && self.peek().unwrap().ttype != TType::RParen {
                            args.push(self.parse_expr()?);
                        }

                        self.advance(TType::RParen)?;

                        Expr::Tuple(args)
                    }
                    TType::RParen => Expr::Literal(Literal::Unit),
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
                                Expr::Constr(x.to_string(), args)
                            } else {
                                Expr::Call(Box::new(func), args)
                            }
                        } else {
                            Expr::Call(Box::new(func), args)
                        }
                    }
                    _ => return error!("{}:{} | Unexpected Literal.", subroot.line, subroot.col),
                }
            }
            TType::RParen => {
                return error!(
                    "{}:{} | Unexpected Closing Parenthese.",
                    root.line, root.col
                )
            }
            _ => return error!("{}:{} | Unexpected Keyword.", root.line, root.col),
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
        let tokens = Lexer::new("foo").proc_tokens()?;
        let ast = Parser::new(tokens).parse()?;

        assert_eq!(ast, vec![Expr::Var("foo".to_string())]);

        Ok(())
    }

    #[test]
    fn call() -> Result<()> {
        let tokens = Lexer::new("(foobar 4 5)").proc_tokens()?;
        let ast = Parser::new(tokens).parse()?;

        assert_eq!(
            ast,
            vec![Expr::Call(
                Box::new(Expr::Var("foobar".to_string())),
                vec![
                    Expr::Literal(Literal::Integer(4)),
                    Expr::Literal(Literal::Integer(5))
                ]
            )]
        );

        Ok(())
    }

    #[test]
    fn literal() -> Result<()> {
        let tokens = Lexer::new("\"foo\" 42 3.1415926535897932 ()").proc_tokens()?;
        let ast = Parser::new(tokens).parse()?;

        assert_eq!(
            ast,
            vec![
                Expr::Literal(Literal::String("foo".to_string())),
                Expr::Literal(Literal::Integer(42)),
                Expr::Literal(Literal::Single(3.1415926535897932)),
                Expr::Literal(Literal::Unit)
            ]
        );

        Ok(())
    }

    #[test]
    fn lambda() -> Result<()> {
        let tokens = Lexer::new("(λ (x y) 5)").proc_tokens()?;
        let ast = Parser::new(tokens).parse()?;

        assert_eq!(
            ast,
            vec![Expr::Lambda(
                vec!["x".to_string(), "y".to_string()],
                Box::new(Expr::Literal(Literal::Integer(5)))
            )]
        );

        Ok(())
    }

    #[test]
    fn def() -> Result<()> {
        let tokens = Lexer::new("(def foo 5)(def 'impure moo jsp)").proc_tokens()?;
        let ast = Parser::new(tokens).parse()?;

        assert_eq!(
            ast,
            vec![
                Expr::Def(
                    "foo".to_string(),
                    Box::new(Expr::Literal(Literal::Integer(5)), false)
                ),
                Expr::Def(
                    "moo".to_string(),
                    Box::new(Expr::Var("jsp".to_string())),
                    true
                )
            ]
        );

        Ok(())
    }

    #[test]
    fn constr() -> Result<()> {
        let tokens = Lexer::new("(Just a)Nothing").proc_tokens()?;
        let ast = Parser::new(tokens).parse()?;

        assert_eq!(
            ast,
            vec![
                Expr::Constr("Just".to_string(), vec![Expr::Var("a".to_string())]),
                Expr::Constr("Nothing".to_string(), vec![])
            ]
        );

        Ok(())
    }

    #[test]
    fn r#enum() -> Result<()> {
        let tokens = Lexer::new("(enum Maybe (Just x) Nil)").proc_tokens()?;
        let ast = Parser::new(tokens).parse()?;

        assert_eq!(
            ast,
            vec![Expr::Enum(
                "Maybe".to_string(),
                table! {"Just".to_string() => 1u8, "Nil".to_string() => 0u8}
            )]
        );

        Ok(())
    }

    #[test]
    fn tuple() -> Result<()> {
        let tokens = Lexer::new("(, a b c)").proc_tokens()?;
        let ast = Parser::new(tokens).parse()?;

        assert_eq!(
            ast,
            vec![Expr::Tuple(vec![
                Expr::Var("a".to_string()),
                Expr::Var("b".to_string()),
                Expr::Var("c".to_string())
            ])]
        );

        Ok(())
    }

    #[test]
    fn load() -> Result<()> {
        let tokens = Lexer::new("(load \"foo\" \"bar\")").proc_tokens()?;
        let ast = Parser::new(tokens).parse()?;

        assert_eq!(
            ast,
            vec![Expr::Load(vec!["foo".to_string(), "bar".to_string()])]
        );

        Ok(())
    }

    #[test]
    fn r#match() -> Result<()> {
        let tokens = Lexer::new("(match foo (bar x)(_ 9))").proc_tokens()?;
        let ast = Parser::new(tokens).parse()?;

        assert_eq!(
            ast,
            vec![Expr::Match(
                Box::new(Expr::Var("foo".to_string())),
                vec![
                    (Pattern::Var("bar".to_string()), Expr::Var("x".to_string())),
                    (
                        Pattern::Var("_".to_string()),
                        Expr::Literal(Literal::Integer(9))
                    )
                ]
            )]
        );

        Ok(())
    }

    #[test]
    fn r#panic() -> Result<()> {
        let tokens = Lexer::new("(panic a)").proc_tokens()?;
        let ast = Parser::new(tokens).parse()?;

        assert_eq!(
            ast,
            vec![Expr::Panic(
                "[1:6] Program panicked at: ".to_string(),
                Box::new(Expr::Var("a".to_string()))
            )]
        );

        Ok(())
    }

    #[test]
    fn begin() -> Result<()> {
        let tokens = Lexer::new("(begin a b c)").proc_tokens()?;
        let ast = Parser::new(tokens).parse()?;

        assert_eq!(
            ast,
            vec![Expr::Begin(vec![
                Expr::Var("a".to_string()),
                Expr::Var("b".to_string()),
                Expr::Var("c".to_string())
            ])]
        );

        Ok(())
    }

    #[test]
    fn quote() -> Result<()> {
        let tokens = Lexer::new("'a").proc_tokens()?;
        let ast = Parser::new(tokens).parse()?;

        assert_eq!(ast, vec![Expr::Lambda(vec![], Box::new(Expr::Var("a".to_string())))]);

        Ok(())
    }

    #[test]
    fn builtins() -> Result<()> {
        let tokens = Lexer::new("(format 5 a)").proc_tokens()?;
        let ast = Parser::new(tokens).parse()?;

        assert_eq!(
            ast,
            vec![Expr::Builtin(
                "format".to_string(),
                vec![
                    Expr::Literal(Literal::Integer(5)),
                    Expr::Var("a".to_string())
                ]
            )]
        );

        Ok(())
    }
}
