use crate::{bug, error, parser::Expr, OrionError, Result};
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Integer(i32),
    Single(f32),
    String(String),
    Lambda(Vec<HashMap<String, Value>>, String, Expr),
    Unit,
    Constr(usize, Vec<Value>),
}

pub struct Interpreter {
    input: Vec<Expr>,
    scopes: Vec<HashMap<String, Value>>,
    name_idx: HashMap<String, (usize, String)>,
    variants: Vec<u8>,
}

impl Interpreter {
    pub fn new(input: Vec<Expr>) -> Self {
        Self {
            input,
            scopes: vec![HashMap::new()],
            name_idx: HashMap::new(),
            variants: vec![],
        }
    }
    fn get_lit_val(&self, val: &Value) -> String {
        match val {
            Value::Integer(i) => format!("{}", i),
            Value::Single(f) => format!("{}", f),
            Value::String(s) => format!("{}", s),
            Value::Lambda(_, x, _) => format!("λ ({})", x),
            Value::Unit => "()".to_string(),
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
    pub fn interpret(&mut self) -> Result<Value> {
        let toret = self.eval_expressions(&(self.input.clone()))?;
        Ok(toret)
    }
    pub fn update_ast(&mut self, ast: Vec<Expr>) {
        self.input = ast;
    }
    fn eval_expressions(&mut self, expressions: &Vec<Expr>) -> Result<Value> {
        for (idx, expr) in expressions.into_iter().enumerate() {
            if idx == expressions.len() - 1 {
                return self.eval_expr(expr, None);
            } else {
                self.eval_expr(expr, None)?;
            }
        }

        Ok(Value::Unit) // Should not be called
    }
    fn eval_expr(
        &mut self,
        expr: &Expr,
        custom_scope: Option<&Vec<HashMap<String, Value>>>,
    ) -> Result<Value> {
        match expr {
            Expr::Def(name, value) => {
                let valued = self.eval_expr(value, None)?;

                if self.scopes.last().unwrap().contains_key(name) {
                    error!("Constant is already in scope: {}.", name)
                } else {
                    self.scopes
                        .last_mut()
                        .unwrap()
                        .insert(name.to_string(), valued);
                    Ok(Value::Unit)
                }
            }
            Expr::Integer(i) => Ok(Value::Integer(*i)),
            Expr::Single(f) => Ok(Value::Single(*f)),
            Expr::String(s) => Ok(Value::String(s.to_string())),
            Expr::Unit => Ok(Value::Unit),
            Expr::Lambda(arg, body) => Ok(Value::Lambda(
                self.scopes.clone(),
                arg.to_string(),
                (**body).clone(),
            )),
            Expr::Call(function, argument) => {
                let arg = self.eval_expr(&**argument, None)?;

                // TEMPORARY, JUST FOR TESTING
                if let Expr::Var(v) = &**function {
                    if v.as_str() == "print" {
                        println!("{}", self.get_lit_val(&arg));
                        return Ok(Value::Unit);
                    }
                }

                let func = self.eval_expr(&**function, None)?;

                if let Value::Lambda(scopes, argument, body) = func {
                    let mut new_scopes = scopes;
                    new_scopes.push(HashMap::new());

                    new_scopes.last_mut().unwrap().insert(argument, arg);

                    self.eval_expr(&body, Some(&new_scopes))?;

                    Ok(Value::Unit)
                } else {
                    error!(
                        "Attempted to use an expression of type {} as a Function.",
                        match func {
                            Value::Integer(_) => "Integer",
                            Value::String(_) => "String",
                            Value::Unit => "Unit",
                            Value::Single(_) => "Single",
                            Value::Constr(idx, _) => self
                                .name_idx
                                .iter()
                                .find_map(|(k, v)| if (*v).0 == idx { Some(k) } else { None })
                                .unwrap()
                                .as_str(),
                            _ => bug!("PREVIOUSLY_MATCHED_EXPRESSION_TRIGGERED_MATCH_ARM"),
                        }
                    )
                }
            }
            Expr::Enum(name, variants) => {
                for (variant, containing) in variants {
                    if self.name_idx.contains_key(variant) {
                        return error!(
                            "Attempted to redefine an existing enum variant: {}.",
                            variant
                        );
                    }

                    let length = self.variants.len();
                    self.name_idx
                        .insert(variant.to_string(), (length, name.to_string()));
                    self.variants.push(*containing);
                }

                Ok(Value::Unit)
            }
            Expr::Constr(name, args) => {
                if !self.name_idx.contains_key(name) {
                    error!("Attempted to use an undefined enum variant: {}.", name)
                } else {
                    let idx = self.name_idx[name].0;

                    if args.len() as u8 != self.variants[idx] {
                        error!(
                            "This enum variant takes {} values but {} were supplied.",
                            self.variants[idx],
                            args.len()
                        )
                    } else {
                        let mut values = vec![];

                        for arg in args {
                            values.push(self.eval_expr(arg, None)?);
                        }

                        Ok(Value::Constr(idx, values))
                    }
                }
            }
            Expr::Var(var) => {
                for scope in match custom_scope {
                    Some(s) => s.iter().rev(),
                    None => self.scopes.iter().rev(),
                } {
                    if scope.contains_key(var) {
                        return Ok(scope[var].clone());
                    }
                }

                error!("Constant not in scope: {}.", var)
            }
        }
    }
}
