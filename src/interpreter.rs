use crate::{
    bug, error,
    lexer::{Lexer},
    parser::{Parser, Expr, Literal, Pattern},
    OrionError, Result,
};
use std::{cmp::Ordering, path::Path, fs, collections::HashMap, env};

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Integer(i32),
    Single(f32),
    String(String),
    Lambda(Vec<HashMap<String, Value>>, String, Expr),
    Unit,
    Constr(usize, Vec<Value>),
    Tuple(Vec<Value>),
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
    fn get_val_type(&self, val: &Value) -> String {
        match val {
            Value::Integer(_) => "Integer".to_string(),
            Value::Single(_) => "Single".to_string(),
            Value::String(_) => "String".to_string(),
            Value::Lambda(_, _,_) => "Lambda".to_string(),
            Value::Unit => "Unit".to_string(),
            Value::Constr(idx, _) => {
                self.name_idx.iter().find_map(|(_, (i, master))| if idx == i { Some ( master.to_string() ) } else { None }).unwrap()

            }
            Value::Tuple(vals) => {
                let mut t = "(".to_string();

                for val in vals {
                    t.push_str(&self.get_val_type(&val));
                }

                format!("{})", t)
            }
        }
    }
    fn get_lit_val(&self, val: &Value) -> String {
        match val {
            Value::Integer(i) => format!("{}", i),
            Value::Single(f) => format!("{}", f),
            Value::String(s) => format!("{}", s),
            Value::Lambda(_, x, _) => format!("λ{}", x),
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
    fn eval_expressions(&mut self, expressions: &Vec<Expr>) -> Result<Value> {
        for (idx, expr) in expressions.into_iter().enumerate() {
            if idx == expressions.len() - 1 {
                return Ok(self.eval_expr(expr, None)?);
            } else {
                self.eval_expr(expr, None)?;
            }
        }

        Ok(Value::Unit) // Should not be called
    }
    fn eval_def(&mut self, name: &String, value: &Expr) -> Result<Value> {
        let valued = self.eval_expr(value, None)?;

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
    fn eval_literal(&mut self, literal: &Literal) -> Result<Value> {
        match literal {
            Literal::Integer(i) => Ok(Value::Integer(*i)),
            Literal::Single(f) => Ok(Value::Single(*f)),
            Literal::String(s) => Ok(Value::String(s.to_string())),
            Literal::Unit => Ok(Value::Unit),
        }
    }
    fn eval_lambda(
        &mut self,
        arg: &String,
        custom_scope: Option<&Vec<HashMap<String, Value>>>,
        body: &Box<Expr>,
        ) -> Result<Value> {
        Ok(Value::Lambda(
                custom_scope.unwrap_or(&self.scopes).clone(),
                arg.to_string(),
                (**body).clone(),
                ))
    }
    fn eval_call(
        &mut self,
        function: &Box<Expr>,
        argument: &Box<Expr>,
        custom_scope: Option<&Vec<HashMap<String, Value>>>,
        ) -> Result<Value> {
        let arg = self.eval_expr(&**argument, custom_scope)?;

        // TEMPORARY, JUST FOR TESTING
        if let Expr::Var(v) = &**function {
            if v.as_str() == "print" {
                println!("{}", self.get_lit_val(&arg));
                return Ok(Value::Unit);
            }
        }

        let func = self.eval_expr(&**function, custom_scope)?;

        if let Value::Lambda(scopes, argument, body) = func {
            let mut new_scopes = scopes;
            new_scopes.push(HashMap::new());

            new_scopes.last_mut().unwrap().insert(argument, arg);
            self.eval_expr(&body, Some(&new_scopes))
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
    fn eval_enum(&mut self, name: &String, variants: &HashMap<String, u8>) -> Result<Value> {
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
    fn eval_constructor(&mut self, name: &String, args: &Vec<Expr>) -> Result<Value> {
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
    fn eval_tuple(&mut self, content: &Vec<Expr>) -> Result<Value> {
        let mut vals = vec![];

        for field in content {
            vals.push(self.eval_expr(field, None)?);
        }

        Ok(Value::Tuple(vals))
    }
    fn eval_var(
        &mut self,
        var: &String,
        custom_scope: Option<&Vec<HashMap<String, Value>>>,
        ) -> Result<Value> {
        for scope in match custom_scope {
            Some(s) => s.iter().rev(),
                None => self.scopes.iter().rev(),
        } {
            if scope.contains_key(var) {
                return Ok(scope[var].clone());
            }
        }

        error!("Literal not in scope: {}.", var)
    }
    fn match_and_bound(
        &mut self,
        patternized: &Pattern,
        pattern: &Pattern,
        custom_scope: Option<&Vec<HashMap<String, Value>>>,
        ) -> Option<HashMap<String, Value>> {
        let mut to_ret = HashMap::new();

        let val = match self.unpatternize(patternized, custom_scope) {
            Ok(v) => v,
            _ => bug!("UNEXPECTED_ERROR"),
        };


        match pattern {
            Pattern::Var(v) => match self.eval_var(v, custom_scope) {
                Ok(v) => {
                    if v == val {
                        Some(to_ret)
                    } else {
                        None
                    }
                }
                _ => {
                    to_ret.insert(v.to_string(), val);
                    Some(to_ret)
                }
            },
            Pattern::Tuple(patterns) => {
                if let Pattern::Tuple(vpat) = patternized {
                    if vpat.len() == patterns.len() {
                        for i in 0..vpat.len() {
                            match self.match_and_bound(&vpat[i], &patterns[i], custom_scope) {
                                Some(sc) => to_ret.extend(sc),
                                None => return None,
                            }
                        }
                        Some(to_ret)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Pattern::Constr(name, patterns) => {
                if let Pattern::Constr(vname, vpat) = patternized {
                    if vname == name {
                        if vpat.len() == patterns.len() {
                            for i in 0..vpat.len() {
                                match self.match_and_bound(&vpat[i], &patterns[i], custom_scope) {
                                    Some(sc) => to_ret.extend(sc),
                                    None => return None,
                                }
                            }
                            Some(to_ret)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => match self.unpatternize(&pattern, custom_scope) {
                Ok(v) => {
                    if v == val {
                        Some(to_ret)
                    } else {
                        None
                    }
                }
                _ => bug!("UNEXPECTED_ERROR"),
            },
        }
    }
    fn eval_match(
        &mut self,
        to_match: &Expr,
        couples: &Vec<(Pattern, Expr)>,
        custom_scope: Option<&Vec<HashMap<String, Value>>>,
        ) -> Result<Value> {
        let to_match = self.eval_expr(to_match, custom_scope)?;

        let patternized = self.patternize(&to_match)?;

        for (pat, expr) in couples {
            match self.match_and_bound(&patternized, &pat, custom_scope) {
                Some(scopes) => {
                    let cloned = self.scopes.clone();
                    let mut new = (*(custom_scope.clone().unwrap_or(&cloned))).clone();
                    new.push(scopes);
                    return self.eval_expr(expr, Some(&new));
                }
                None => continue,
            }
        }

        error!("No pattern can be matched.")
    }
    fn unpatternize(
        &mut self,
        pat: &Pattern,
        custom_scope: Option<&Vec<HashMap<String, Value>>>,
        ) -> Result<Value> {
        Ok(match pat {
            Pattern::Literal(lit) => match lit {
                Literal::Integer(i) => Value::Integer(*i),
                Literal::Single(f) => Value::Single(*f),
                Literal::String(s) => Value::String(s.to_string()),
                Literal::Unit => Value::Unit,
            },
            Pattern::Var(v) => self.eval_var(&v, custom_scope)?,
            Pattern::Tuple(vals) => {
                let mut valued = vec![];

                for val in vals {
                    valued.push(self.unpatternize(val, custom_scope)?);
                }
                Value::Tuple(valued)
            }
            Pattern::Constr(variant, params) => {
                if !self.name_idx.contains_key(variant) {
                    return error!("Attempted to use an undefined enum variant: {}.", &variant);
                }

                let idx = *&self.name_idx[variant].0;

                let mut fields = vec![];
                for param in params {
                    fields.push(self.unpatternize(param, custom_scope)?);
                }

                Value::Constr(idx, fields)
            }
        })
    }
    fn patternize(&mut self, val: &Value) -> Result<Pattern> {
        Ok(match val {
            Value::Integer(i) => Pattern::Literal(Literal::Integer(*i)),
            Value::Single(f) => Pattern::Literal(Literal::Single(*f)),
            Value::String(s) => Pattern::Literal(Literal::String(s.to_string())),
            Value::Unit => Pattern::Literal(Literal::Unit),
            Value::Tuple(vals) => {
                let mut patterned = vec![];

                for val in vals {
                    patterned.push(self.patternize(val)?);
                }

                Pattern::Tuple(patterned)
            }
            Value::Constr(idx, params) => {
                let named = self
                    .name_idx
                    .iter()
                    .find_map(|(k, (val, _))| {
                        if *val == *idx {
                            Some(k.to_string())
                        } else {
                            None
                        }
                    })
                .unwrap();
                let mut patterned = vec![];

                for param in params {
                    patterned.push(self.patternize(param)?);
                }

                Pattern::Constr(named, patterned)
            }
            Value::Lambda(_, _, _) => {
                return error!("Expected Constructor, Tuple or Literal, found Lambda.")
            }
        })
    }
    fn eval_load(&mut self, params: &Vec<String>) -> Result<Value> {

        let lib_link = match env::var("ORION_LIB") {
            Ok(v) => v,
            _ => if cfg!(windows) {
                "C:/Program Files/Orion/lib"
            } else if cfg!(macos) {
                "/usr/local/lib/orion"
            } else {
                "/usr/lib/orion"
            }.to_string()
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

    fn eval_expr(
        &mut self,
        expr: &Expr,
        custom_scope: Option<&Vec<HashMap<String, Value>>>,
        ) -> Result<Value> {
        match expr {
            Expr::Def(name, value) => self.eval_def(name, value),
            Expr::Match(to_match, couples) => self.eval_match(to_match, couples, custom_scope),
            Expr::Literal(literal) => self.eval_literal(literal),
            Expr::Call(function, argument) => self.eval_call(function, argument, custom_scope),
            Expr::Load(params) => self.eval_load(params),
            Expr::Lambda(arg, body) => self.eval_lambda(arg, custom_scope, body),
            Expr::Enum(name, variants) => self.eval_enum(name, variants),
            Expr::Constr(name, args) => self.eval_constructor(name, args),
            Expr::Tuple(content) => self.eval_tuple(content),
            Expr::Var(var) => self.eval_var(var, custom_scope),
            Expr::Add(lh, rh) => {
                let lhs = self.eval_expr(&**lh, custom_scope)?;
                let rhs = self.eval_expr(&**rh, custom_scope)?;

                match lhs {
                    Value::Integer(lh) => match rhs {
                        Value::Integer(rh) => Ok(Value::Integer(lh + rh)),
                        _ => error!("Attempted to add {} to {}.", self.get_val_type(&lhs), self.get_val_type(&rhs)),
                    }
                    Value::Single(lh) => match rhs {
                        Value::Single(rh) => Ok(Value::Single(lh + rh)),
                        _ => error!("Attempted to add {} to {}.", self.get_val_type(&lhs), self.get_val_type(&rhs)),
                    }
                    _ => error!("Expected Single or Integer, found {}.", self.get_val_type(&lhs)),
                }
            }
            Expr::Div(lh, rh) => {
                let lhs = self.eval_expr(&**lh, custom_scope)?;
                let rhs = self.eval_expr(&**rh, custom_scope)?;

                match lhs {
                    Value::Integer(lh) => match rhs {
                        Value::Integer(rh) => if rh == 0 {
                            Ok(Value::Single(std::f32::INFINITY))
                        } else {
                            Ok(Value::Integer(lh / rh))
                        },
                        _ => error!("Attempted to divide {} by {}.", self.get_val_type(&lhs), self.get_val_type(&rhs)),
                    }
                    Value::Single(lh) => match rhs {
                        Value::Single(rh) => if rh == 0. {
                            Ok(Value::Single(std::f32::INFINITY))
                        } else {
                            Ok(Value::Single(lh / rh))
                        },
                        _ => error!("Attempted to divide {} by {}.", self.get_val_type(&lhs), self.get_val_type(&rhs)),
                    }
                    _ => error!("Expected Single or Integer, found {}.", self.get_val_type(&lhs)),
                }

            }
            Expr::Opp(val) => {
                let val = self.eval_expr(val,custom_scope)?;

                match val {
                    Value::Integer(i) => Ok(Value::Integer(0 - i)),
                    Value::Single(r) => Ok(Value::Single(0. - r)),
                    _ => error!("Expected Single or Integer, found {}.", self.get_val_type(&val)),
                }
            }
            Expr::Cmp(lh, rh) => {
                let lhs = self.eval_expr(&**lh, custom_scope)?;
                let rhs = self.eval_expr(&**rh, custom_scope)?;

                match lhs {
                    Value::Integer(lh) => match rhs {
                        Value::Integer(rh) => Ok(Value::Integer(match lh.cmp(&rh) {
                            Ordering::Less => 0,
                            Ordering::Equal => 1,
                            Ordering::Greater => 2,
                        })),
                        _ => error!("Attempted to compare {} with {}.", self.get_val_type(&lhs), self.get_val_type(&rhs)),
                    }
                    Value::Single(lh) => match rhs {
                        Value::Single(rh) => Ok(Value::Integer(match lh.partial_cmp(&rh).unwrap() {
                            Ordering::Less => 0,
                            Ordering::Equal => 1,
                            Ordering::Greater => 2,
                        })),
                        _ => error!("Attempted to compare {} with {}.", self.get_val_type(&lhs), self.get_val_type(&rhs)),
                    }
                    _ => error!("Expected Single or Integer, found {}.", self.get_val_type(&lhs)),
                }

            }


        }
    }
}
