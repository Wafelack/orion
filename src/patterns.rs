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
use crate::{interpreter::{Interpreter, Value}, bug, OrionError, error, Result, parser::{Pattern, Expr, Literal}};
use std::collections::HashMap;

impl Interpreter {
    pub fn match_and_bound(
        &mut self,
        patternized: &Pattern,
        pattern: &Pattern,
        ctx: Option<&Vec<HashMap<String, Value>>>,
        mut to_ret: HashMap<String, Value>,
        ) -> Option<HashMap<String, Value>> {

        let val = match self.unpatternize(patternized, ctx) {
            Ok(v) => v,
            _ => bug!("UNEXPECTED_ERROR"),
        };

        match pattern {
            Pattern::Var(v) => {

                let mut new_ctx = match ctx {
                    Some(c) => c.clone(),
                    None => self.scopes.clone(),
                };

                new_ctx.push(to_ret.clone());

                match self.eval_var(v, Some(&new_ctx)) {
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
                }
            }
            Pattern::Tuple(patterns) => {
                if let Pattern::Tuple(vpat) = patternized {
                    if vpat.len() == patterns.len() {
                        for i in 0..vpat.len() {
                            match self.match_and_bound(&vpat[i], &patterns[i], ctx, to_ret.clone()) {
                                Some(sc) => to_ret.extend(sc),
                                None => return None,
                            }
                        }
                        return Some(to_ret);
                    }
                }
                return None;
            }
            Pattern::Constr(name, patterns) => {
                if let Pattern::Constr(vname, vpat) = patternized {
                    if vname == name {
                        if vpat.len() == patterns.len() {
                            for i in 0..vpat.len() {
                                match self.match_and_bound(&vpat[i], &patterns[i], ctx, to_ret.clone()) {
                                    Some(sc) => to_ret.extend(sc),
                                    None => return None,
                                }
                            }
                            return Some(to_ret);
                        } 

                    }
                }
                return None;
            }
            _ => match self.unpatternize(&pattern, ctx) {
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
    pub fn eval_match(
        &mut self,
        to_match: &Expr,
        couples: &Vec<(Pattern, Expr)>,
        ctx: Option<&Vec<HashMap<String, Value>>>,
        ) -> Result<Value> {
        let to_match = self.eval_expr(to_match, ctx)?;

        let patternized = self.patternize(&to_match)?;

        for (pat, expr) in couples {
            match self.match_and_bound(&patternized, &pat, ctx, HashMap::new()) {
                Some(scopes) => {
                    let mut new = match ctx {
                        Some(s) => (*s).clone(),
                        None => self.scopes.clone(),
                    };
                    new.push(scopes);
                    return self.eval_expr(expr, Some(&new));
                }
                None => continue,
            }
        }

        error!("No pattern can be matched.")
    }
    pub fn unpatternize(
        &mut self,
        pat: &Pattern,
        ctx: Option<&Vec<HashMap<String, Value>>>,
        ) -> Result<Value> {
        Ok(match pat {
            Pattern::Literal(lit) => match lit {
                Literal::Integer(i) => Value::Integer(*i),
                Literal::Single(f) => Value::Single(*f),
                Literal::String(s) => Value::String(s.to_string()),
                Literal::Unit => Value::Unit,
            },
            Pattern::Var(v) => self.eval_var(&v, ctx)?,
            Pattern::Tuple(vals) => {
                let mut valued = vec![];

                for val in vals {
                    valued.push(self.unpatternize(val, ctx)?);
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
                    fields.push(self.unpatternize(param, ctx)?);
                }

                Value::Constr(idx, fields)
            }
        })
    }
    pub fn patternize(&mut self, val: &Value) -> Result<Pattern> {
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
            x => {
                return error!("Expected Constructor, Tuple or Literal, found {}.", self.get_val_type(x));
            }
        })
    }

}
