/*
 *  Copyright (C) 2021, Wafelack <wafelack@protonmail.com>
 *
 *  ------------------------------------------------------
 *
 *     This file is part of Orion.
 *
 *  Orion is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *  Orion is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Orion.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::{
    error,
    lexer::Lexer,
    parser::{Expr, Literal, Parser},
    builtins::ArgsLength,
    OrionError, Result,
};
use std::{collections::HashMap, env, fs, path::Path};

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Integer(i32),
    Single(f32),
    String(String),
    Lambda(Vec<HashMap<String, Value>>, String, Expr),
    Unit,
    Constr(usize, Vec<Value>),
    Quote(Expr),
    Tuple(Vec<Value>),
}

pub struct Interpreter {
    input: Vec<Expr>,
    pub scopes: Vec<HashMap<String, Value>>,
    pub name_idx: HashMap<String, (usize, String)>,
    pub variants: Vec<u8>,
    pub builtins: HashMap<String, (fn(&mut Interpreter, Vec<Value>,Option<&Vec<HashMap<String, Value>>>) -> Result<Value>, ArgsLength)>,
    imported: Vec<String>,
}

impl Interpreter {
    pub fn new(input: Vec<Expr>) -> Self {
        Self {
            input,
            scopes: vec![HashMap::new()],
            name_idx: HashMap::new(),
            variants: vec![],
            builtins: HashMap::new(),
            imported: vec![],
        }
    }
    fn register_builtin(&mut self, builtin: impl ToString, callback: fn(&mut Interpreter, Vec<Value>, Option<&Vec<HashMap<String, Value>>>) -> Result<Value>, length: ArgsLength) {
        self.builtins.insert(builtin.to_string(), (callback, length));
    }
    pub fn get_val_type(&self, val: &Value) -> String {
        match val {
            Value::Quote(_) => "Quote".to_string(),
            Value::Integer(_) => "Integer".to_string(),
            Value::Single(_) => "Single".to_string(),
            Value::String(_) => "String".to_string(),
            Value::Lambda(_, _, _) => "Lambda".to_string(),
            Value::Unit => "Unit".to_string(),
            Value::Constr(idx, _) => self
                .name_idx
                .iter()
                .find_map(|(_, (i, master))| {
                    if idx == i {
                        Some(master.to_string())
                    } else {
                        None
                    }
                })
            .unwrap(),
            Value::Tuple(vals) => {
                let mut t = "(".to_string();

                for val in vals {
                    t.push_str(&self.get_val_type(&val));
                }

                format!("{})", t)
            }
        }
    }
    pub fn get_lit_val(&self, val: &Value) -> String {
        match val {
            Value::Integer(i) => format!("{}", i),
            Value::Quote(expr) => format!("Quote{}", expr.get_type()),
            Value::Single(f) => format!("{}", f),
            Value::String(s) => format!("{}", s),
            Value::Lambda(_, x, _) => format!("<#lambda {}>", x),
            Value::Unit => "()".to_string(),
            Value::Tuple(vals) => format!(
                "({})",
                vals.iter()
                .map(|v| self.get_lit_val(&v))
                .collect::<Vec<_>>()
                .join(", ")
                ),
                Value::Constr(idx, vals) => {
                    let name = self
                        .name_idx
                        .iter()
                        .find_map(|(name, (i, _))| if *i == *idx { Some(name) } else { None })
                        .unwrap();

                    if vals.is_empty() {
                        format!("{}", name)
                    } else {
                        format!(
                            "({} {})",
                            name,
                            vals.iter()
                            .map(|v| self.get_lit_val(v))
                            .collect::<Vec<String>>()
                            .join(" ")
                            )
                    }
                }
        }
    }
    pub fn update_ast(&mut self, ast: Vec<Expr>) {
        self.input = ast;
    }
    pub fn eval_def(&mut self, name: &String, value: &Expr, ctx: Option<&Vec<HashMap<String, Value>>>) -> Result<Value> {
        let valued = self.eval_expr(value, ctx)?;

        if self.scopes.last().unwrap().contains_key(name) {
            error!("Literal is already in scope: {}.", name)
        } else {
            self.scopes
                .last_mut()
                .unwrap()
                .insert(name.to_string(), valued);
            Ok(Value::Unit)
        }
    }
    pub fn eval_literal(&mut self, literal: &Literal) -> Result<Value> {
        match literal {
            Literal::Integer(i) => Ok(Value::Integer(*i)),
            Literal::Single(f) => Ok(Value::Single(*f)),
            Literal::String(s) => Ok(Value::String(s.to_string())),
            Literal::Unit => Ok(Value::Unit),
        }
    }
    pub fn eval_var(
        &mut self,
        var: &String,
        ctx: Option<&Vec<HashMap<String, Value>>>,
        ) -> Result<Value> {
        let scopes = match ctx {
            Some(s) => {
                s
            },
            None => &self.scopes,
        }.iter().rev();

        for scope in scopes {
            if scope.contains_key(var) {
                return Ok(scope[var].clone());
            }
        }

        error!("Literal not in scope: {}.", var)
    }
    pub fn eval_load(&mut self, params: &Vec<String>) -> Result<Value> {
        let lib_link = match env::var("ORION_LIB") {
            Ok(v) => v,
            Err(_) => return error!("ORION_LIB variable not found.")
        };

        for param in params {
            let lib_path = &format!("{}/{}", lib_link, param);

            let (content, fname) = if Path::new(lib_path).exists() {
                match fs::read_to_string(lib_path) {
                    Ok(c) => (c, lib_path),
                    _ => return error!("Failed to read file: {}.", lib_path),
                }
            } else if Path::new(&param).exists() {
                match fs::read_to_string(param) {
                    Ok(c) => (c, param),
                    _ => return error!("Failed to read file: {}.", &param),
                }
            } else {
                return error!("File not found: {}.", param);
            };

            if !self.imported.contains(&fname) {
                self.imported.push(fname.to_string());
            } else {
                continue;
            }

            self.eval_expressions(&Parser::new(Lexer::new(content).proc_tokens()?).parse()?, None)?;
        }

        Ok(Value::Unit)
    }

    pub fn eval_expr(
        &mut self,
        expr: &Expr,
        ctx: Option<&Vec<HashMap<String, Value>>>,
        ) -> Result<Value> {
        match expr {
            Expr::Quote(expr) => Ok(Value::Quote((**expr).clone())),
            Expr::Def(name, value) => self.eval_def(name, value, ctx),
            Expr::Match(to_match, couples) => self.eval_match(to_match, couples, ctx),
            Expr::Literal(literal) => self.eval_literal(literal),
            Expr::Call(function, argument) => self.eval_call(function, argument, ctx),
            Expr::Load(params) => self.eval_load(params),
            Expr::Begin(exprs) => {
                let mut new_scope = ctx.and_then(|c| Some(c.clone())).unwrap_or(self.scopes.clone());
                new_scope.push(HashMap::new());
                self.eval_expressions(exprs, ctx)
            }
            Expr::Lambda(arg, body) => self.eval_lambda(arg, ctx, body),
            Expr::Enum(name, variants) => self.eval_enum(name, variants),
            Expr::Constr(name, args) => self.eval_constructor(name, args, ctx),
            Expr::Tuple(content) => self.eval_tuple(content, ctx),
            Expr::Var(var) => self.eval_var(var, ctx),
            Expr::Panic(prefix, content) => {
                let valued = self.eval_expr(&**content, ctx)?;
                eprintln!("{}{}", prefix, self.get_lit_val(&valued));
                std::process::exit(1);
            }
            Expr::Builtin(builtin, args) => self.eval_builtin(builtin.to_string(), args.clone(), ctx)
        }
    }

    pub fn eval_expressions(&mut self, expressions: &Vec<Expr>, ctx: Option<&Vec<HashMap<String, Value>>>) -> Result<Value> { 
        for (idx, expr) in expressions.into_iter().enumerate() {
            if idx == expressions.len() - 1 {
                return Ok(self.eval_expr(expr, ctx)?);
            } else {
                self.eval_expr(expr, ctx)?;
            }
        }

        Ok(Value::Unit) // Should not be called
    }

    pub fn interpret(&mut self, repl: bool, no_prelude: bool) -> Result<Value> {

        self.register_builtin("+", Self::add, ArgsLength::OrMore(2));
        self.register_builtin("-", Self::sub, ArgsLength::OrMore(2));
        self.register_builtin("*", Self::mul, ArgsLength::OrMore(2));
        self.register_builtin("/", Self::div, ArgsLength::OrMore(2));
        self.register_builtin("!", Self::opp, ArgsLength::Fixed(1));
        self.register_builtin("_cmp", Self::cmp, ArgsLength::Fixed(2));
        self.register_builtin("cos", Self::cos, ArgsLength::Fixed(1));
        self.register_builtin("sin", Self::sin, ArgsLength::Fixed(1));
        self.register_builtin("tan", Self::tan, ArgsLength::Fixed(1));
        self.register_builtin("acos", Self::acos, ArgsLength::Fixed(1));
        self.register_builtin("asin", Self::asin, ArgsLength::Fixed(1));
        self.register_builtin("atan", Self::atan, ArgsLength::Fixed(1));

        // Impure zone
        self.register_builtin("putStr", Self::put_str, ArgsLength::Fixed(1));
        self.register_builtin("putStrLn", Self::put_str_ln, ArgsLength::Fixed(1));
        self.register_builtin("write", Self::write, ArgsLength::Fixed(2));
        self.register_builtin("getLine", Self::get_line, ArgsLength::Fixed(0));

        self.register_builtin("format", Self::format, ArgsLength::OrMore(1));

        self.register_builtin("unquote", Self::unquote, ArgsLength::Fixed(1));

        // Prelude
        if !no_prelude {
            self.eval_load(&vec!["prelude.orn".to_string()])?;
        }

        let toret = self.eval_expressions(&(self.input.clone()), None)?;
        if repl {
            let to_p = self.get_lit_val(&toret);

            if to_p.as_str() != "()" {
                println!("{}", to_p)
            }
        }
        Ok(toret)
    }


}
