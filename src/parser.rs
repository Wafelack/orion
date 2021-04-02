use crate::{
    bug, error,
    lexer::{TType, Token},
    OrionError, Result,
};
use std::{collections::HashMap, mem::discriminant};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Var(String),                //
    Call(Box<Expr>, Box<Expr>), //
    Lambda(String, Box<Expr>),  //
    Literal(Literal),
    Def(String, Box<Expr>), //
    Constr(String, Vec<Expr>),
    Enum(String, HashMap<String, u8>),
    Tuple(Vec<Expr>),
    Match(Box<Expr>, Vec<(Pattern, Expr)>),
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
            let previous = &self.input[self.current];
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
        self.input.len() != 1 && self.current + 1 >= self.input.len()
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
                } else if first_char(&v).is_ascii_lowercase() || v.as_str() == "_" {
                    Pattern::Var(v.to_string())
                } else {
                    return error!("{}:{} | Invalid variable name: {}.", root.line, root.col, v);
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

                        if !self.is_at_end() {
                            self.advance(TType::RParen)?;
                        }

                        Pattern::Tuple(args)
                    }
                    TType::LParen | TType::Ident(_) => {
                        let mut args = vec![];
                        while !self.is_at_end() && self.peek().unwrap().ttype != TType::RParen {
                            args.push(self.parse_pattern()?);
                        }

                        if !self.is_at_end() {
                            self.advance(TType::RParen)?;
                        }

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
                } else if first_char(&v).is_ascii_lowercase() || v.as_str() == "_" {
                    Expr::Var(v.to_string())
                } else {
                    return error!("{}:{} | Invalid variable name: {}.", root.line, root.col, v);
                }
            }
            TType::LParen => {
                let subroot = self.pop()?;

                match &subroot.ttype {
                    TType::Def => {
                        let raw_name = self.advance(TType::Ident("".to_owned()))?;
                        let name = if let TType::Ident(n) = raw_name.ttype {
                            n
                        } else {
                            bug!("UNEXPECTED_NON_IDENTIFIER");
                        };

                        if !first_char(&name).is_ascii_lowercase() {
                            return error!(
                                "{}:{} | Literal names have to start with a lowercase letter.",
                                raw_name.line, raw_name.col
                            );
                        }

                        let value = self.parse_expr()?;

                        if !self.is_at_end() {
                            self.advance(TType::RParen)?;
                        }

                        Expr::Def(name, Box::new(value))
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

                        if !self.is_at_end() {
                            self.advance(TType::RParen)?;
                        }

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
                                    ident
                                } else {
                                    bug!("What is this thing doing here ?");
                                }
                            })
                            .collect::<Vec<_>>();

                        let mut body = self.parse_expr()?;

                        for arg in args.into_iter().rev() {
                            body = Expr::Lambda(arg.to_string(), Box::new(body));
                        }
                        if !self.is_at_end() {
                            self.advance(TType::RParen)?;
                        }
                        body
                    }
                    TType::Tuple => {
                        let mut args = vec![];
                        while !self.is_at_end() && self.peek().unwrap().ttype != TType::RParen {
                            args.push(self.parse_expr()?);
                        }

                        if !self.is_at_end() {
                            self.advance(TType::RParen)?;
                        }

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

                        if !self.is_at_end() {
                            self.advance(TType::RParen)?;
                        }

                        if let TType::Ident(x) = &subroot.ttype {
                            if first_char(x).is_ascii_uppercase() {
                                Expr::Constr(x.to_string(), args)
                            } else if first_char(&x).is_ascii_lowercase() {
                                args.into_iter().fold(func, |root, elem| {
                                    Expr::Call(Box::new(root), Box::new(elem))
                                })
                            } else {
                                return error!(
                                    "{}:{} | Invalid variable name: {}.",
                                    subroot.line, subroot.col, x
                                );
                            }
                        } else {
                            args.into_iter().fold(func, |root, elem| {
                                Expr::Call(Box::new(root), Box::new(elem))
                            })
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
