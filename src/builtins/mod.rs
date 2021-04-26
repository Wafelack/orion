pub mod arithmetic;
pub mod io;
pub mod string;
pub mod misc;

pub enum ArgsLength {
    OrMore(usize),
    Fixed(usize),    
}

impl ArgsLength {
    pub fn display(&self) -> String {
        match self {
            Self::OrMore(u) => format!("{} or more", u),
            Self::Fixed(u) => format!("{}", u),

        }
    }
    pub fn contains(&self, length: usize) -> bool {
        match self  {
            Self::OrMore(u) => length >= *u,
            Self::Fixed(u) => *u == length,
        }
    }
}

use crate::{interpreter::{Interpreter, Value}, error, Result, OrionError, parser::Expr};
use std::collections::HashMap;

impl Interpreter {
    pub fn eval_builtin(&mut self, name: String, args: Vec<Expr>, ctx: Option<&Vec<HashMap<String, Value>>>) -> Result<Value> {
        if self.builtins.contains_key(&name) {
            let length = &self.builtins[&name].1;
            if length.contains(args.len()) {
                let mut argv = vec![];
                for arg in args {
                    argv.push(self.eval_expr(&arg, ctx)?);
                }
                self.builtins[&name].0(self, argv, ctx)
            } else {
                error!("Builtin `{}` takes {} arguments, but {} arguments were supplied.", name, length.display(), args.len())
            }
        } else {
            error!("Builtin {} is not registered !", name)
        }
    }

}
