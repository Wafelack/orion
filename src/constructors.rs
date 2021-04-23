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
use crate::{interpreter::{Interpreter, Value}, OrionError, error, bug, Result, parser::{Expr, Pattern}};
use std::collections::HashMap;

impl Interpreter {
    pub fn eval_constructor(&mut self, name: &String, args: &Vec<Expr>, custom_scope: Option<&Vec<HashMap<String, Value>>>) -> Result<Value> {
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
                    values.push(self.eval_expr(arg, custom_scope)?);
                }

                Ok(Value::Constr(idx, values))
            }
        }
    }
    pub fn eval_tuple(&mut self, content: &Vec<Expr>, custom_scope: Option<&Vec<HashMap<String, Value>>>) -> Result<Value> {
        let mut vals = vec![];

        for field in content {
            vals.push(self.eval_expr(field, custom_scope)?);
        }

        Ok(Value::Tuple(vals))
    }

    pub fn match_and_bound(
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
                                match self.match_and_bound(&vpat[i], &patterns[i], custom_scope) {
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
    pub fn eval_enum(&mut self, name: &String, variants: &HashMap<String, u8>) -> Result<Value> {
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
}
