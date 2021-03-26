use crate::{Result, bug, error, OrionError, lexer::{Token, TType}};
use std::mem::discriminant;

#[derive(Debug, Clone)]
pub enum Expr {
    Var(String),
    Call(Vec<Expr>),
    Lambda(String, Box<Expr>),
    Integer(i32),
    Single(f32),
    Boolean(bool),
    Unit,
    String(String),
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
            println!("{:?}", &popped);
            error!("{}:{} | Expected {}, found {}.", popped.line, popped.col, expected.get_type(), popped.ttype.get_type()) 
        } else {
            Ok(popped)
        }

    }
    fn pop(&mut self) -> Result<Token> {
        if self.current + 1 >= self.input.len() {
            let previous = &self.input[self.current];
            error!("{}:{} | Unfinished expression.", previous.line, previous.col)
        } else {
            self.current += 1;
            Ok(self.input[self.current - 1].clone())
        }
    }
    fn peek(&self) -> Option<Token> {
        self.input.iter().nth(self.current).and_then(|t| Some(t.clone()))
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }
    fn advance_many(&mut self, expected: TType) -> Result<Vec<Token>> {
        let mut toret = vec![];

        while !self.is_at_end() && &self.peek().unwrap().ttype == &expected {
            toret.push(self.advance(expected.clone())?);
        }

        toret.push(self.advance(expected)?);

        Ok(toret)
    }
    fn parse_expr(&mut self) -> Result<Expr> {

        let root = self.pop()?;

        Ok(match &root.ttype {
            TType::Str(s) => Expr::String(s.to_string()),
            TType::Float(f) => Expr::Single(*f),
            TType::Number(i) => Expr::Integer(*i),
            TType::Bool(b) => Expr::Boolean(*b),
            TType::Ident(v) => Expr::Var(v.to_string()),
            TType::LParen => {
                let subroot = self.pop()?;

                match &subroot.ttype {
                    TType::LParen => self.parse_expr()?,
                    TType::Ident(ident) => match ident.as_str() {
                        "lambda" | "λ" => {
                            self.advance(TType::LParen)?;
                            let args = self.advance_many(TType::Ident("".to_owned()))?;
                            self.advance(TType::RParen)?;

                            let mut args = args.iter().map(|e| if let TType::Ident(ident) = &e.ttype {
                                ident
                            } else {
                                bug!("Qu'est-ce que ça fout là ?");
                            }).collect::<Vec<_>>();

                            let mut body = self.parse_expr()?;

                            for arg in args.into_iter().rev() {
                                body = Expr::Lambda(arg.to_string(), Box::new(body)); 
                            }

                            self.advance(TType::RParen)?;

                            body
                        }
                        _ => unimplemented!(),
                    }
                    TType::RParen => Expr::Unit,
                    _ => return error!("{}:{} | Expected Closing Parenthese, Opening Parenthese or Identifier, found {}.", subroot.line, subroot.col, subroot.ttype.get_type()),
                }
            }
        TType::RParen => return error!("{}:{} | Unexpected Closing Parenthese.", root.line, root.col)
    })
}

pub fn parse(&mut self) -> Result<Vec<Expr>> {

    while self.current < self.input.len() {
        self.parse_expr()?;
    }


    Ok(self.output.clone())
}
}
