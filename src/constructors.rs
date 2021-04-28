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
use crate::{interpreter::{Interpreter, Value}, OrionError, error, Result, parser::Expr};
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
