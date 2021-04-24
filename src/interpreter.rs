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
    builtins: HashMap<String, (fn(&mut Interpreter, Vec<Value>) -> Result<Value>, ArgsLength)>,
}

impl Interpreter {
    pub fn new(input: Vec<Expr>) -> Self {
        Self {
            input,
            scopes: vec![HashMap::new()],
            name_idx: HashMap::new(),
            variants: vec![],
            builtins: HashMap::new(),
        }
    }
    fn register_builtin(&mut self, builtin: impl ToString, callback: fn(&mut Interpreter, Vec<Value>) -> Result<Value>, length: ArgsLength) {
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
            Value::Quote(expr) => format!("'{:?}", expr),
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
    pub fn interpret(&mut self, repl: bool) -> Result<Value> {
        let toret = self.eval_expressions(&(self.input.clone()))?;
        if repl {
            let to_p = self.get_lit_val(&toret);

            if to_p.as_str() != "()" {
                println!("{}", to_p)
            }
        }
        Ok(toret)
    }
    pub fn update_ast(&mut self, ast: Vec<Expr>) {
        self.input = ast;
    }
    pub fn eval_expressions(&mut self, expressions: &Vec<Expr>) -> Result<Value> {

        self.register_builtin("+", Self::add, ArgsLength::OrMore(2));
        self.register_builtin("-", Self::sub, ArgsLength::OrMore(2));
        self.register_builtin("*", Self::mul, ArgsLength::OrMore(2));
        self.register_builtin("/", Self::div, ArgsLength::OrMore(2));
        self.register_builtin("!", Self::opp, ArgsLength::Fixed(1));
        self.register_builtin("_cmp", Self::cmp, ArgsLength::Fixed(2));

        // Impure zone
        self.register_builtin("putStr", Self::put_str, ArgsLength::Fixed(1));
        self.register_builtin("putStrLn", Self::put_str_ln, ArgsLength::Fixed(1));
        self.register_builtin("write", Self::write, ArgsLength::Fixed(2));
        self.register_builtin("getLine", Self::get_line, ArgsLength::Fixed(0));

        self.register_builtin("format", Self::format, ArgsLength::OrMore(1));

        for (idx, expr) in expressions.into_iter().enumerate() {
            if idx == expressions.len() - 1 {
                return Ok(self.eval_expr(expr, None)?);
            } else {
                self.eval_expr(expr, None)?;
            }
        }

        Ok(Value::Unit) // Should not be called
    }
    pub fn eval_builtin(&mut self, name: String, args: Vec<Expr>, custom_scope: Option<&Vec<HashMap<String, Value>>>) -> Result<Value> {
        if self.builtins.contains_key(&name) {
            let length = &self.builtins[&name].1;
            if length.contains(args.len()) {
                let mut argv = vec![];
                for arg in args {
                    argv.push(self.eval_expr(&arg, custom_scope)?);
                }
                self.builtins[&name].0(self, argv)
            } else {
                error!("Builtin `{}` takes {} arguments, but {} arguments were supplied.", name, length.display(), args.len())
            }
        } else {
            error!("Builtin {} is not registered !", name)
        }
    }
    pub fn eval_def(&mut self, name: &String, value: &Expr, custom_scope: Option<&Vec<HashMap<String, Value>>>) -> Result<Value> {
        let valued = self.eval_expr(value, custom_scope)?;

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
        custom_scope: Option<&Vec<HashMap<String, Value>>>,
        ) -> Result<Value> {
        let scopes = match custom_scope {
            Some(s) => {
                s
            },
            None => &self.scopes,
        }.iter().rev();

        for scope in scopes {
            if scope.contains_key(var) {
                if let Value::Quote(expr) = scope[var].clone() {
                    return self.eval_expr(&expr, custom_scope);
                } else {
                    return Ok(scope[var].clone());
                }
            }
        }

        error!("Literal not in scope: {}.", var)
    }
    pub fn eval_load(&mut self, params: &Vec<String>) -> Result<Value> {
        let lib_link = match env::var("ORION_LIB") {
            Ok(v) => v,
            _ => if cfg!(windows) {
                "C:/Program Files/Orion/lib".to_string()
            } else {
                let home = match env::var("HOME") {
                    Ok(h) => h,
                    Err(_) => return error!("Cannot find $HOME variable."),
                };

                format!("{}/.orion/lib/", home)
            }
            .to_string(),
        };

        for param in params {
            let lib_path = &format!("{}/{}", lib_link, param);

            let content = if Path::new(lib_path).exists() {
                match fs::read_to_string(lib_path) {
                    Ok(c) => c,
                    _ => return error!("Failed to read file: {}.", lib_path),
                }
            } else if Path::new(&param).exists() {
                match fs::read_to_string(param) {
                    Ok(c) => c,
                    _ => return error!("Failed to read file: {}.", lib_path),
                }
            } else {
                return error!("File not found: {}.", param);
            };

            self.eval_expressions(&Parser::new(Lexer::new(content).proc_tokens()?).parse()?)?;
        }

        Ok(Value::Unit)
    }

    pub fn eval_expr(
        &mut self,
        expr: &Expr,
        custom_scope: Option<&Vec<HashMap<String, Value>>>,
        ) -> Result<Value> {
        match expr {
            Expr::Quote(expr) => Ok(Value::Quote((**expr).clone())),
            Expr::Def(name, value) => self.eval_def(name, value, custom_scope),
            Expr::Match(to_match, couples) => self.eval_match(to_match, couples, custom_scope),
            Expr::Literal(literal) => self.eval_literal(literal),
            Expr::Call(function, argument) => self.eval_call(function, argument, custom_scope),
            Expr::Load(params) => self.eval_load(params),
            Expr::Begin(exprs) => {
                self.scopes.push(HashMap::new());
                let toret = self.eval_expressions(exprs);
                self.scopes.pop();
                toret
            }
            Expr::Lambda(arg, body) => self.eval_lambda(arg, custom_scope, body),
            Expr::Enum(name, variants) => self.eval_enum(name, variants),
            Expr::Constr(name, args) => self.eval_constructor(name, args, custom_scope),
            Expr::Tuple(content) => self.eval_tuple(content, custom_scope),
            Expr::Var(var) => self.eval_var(var, custom_scope),
            Expr::Panic(prefix, content) => {
                let valued = self.eval_expr(&**content, custom_scope)?;
                eprintln!("{}{}", prefix, self.get_lit_val(&valued));
                std::process::exit(1);
            }
            Expr::Builtin(builtin, args) => self.eval_builtin(builtin.to_string(), args.clone(), custom_scope)
        }
    }
}
