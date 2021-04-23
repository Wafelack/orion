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
use crate::{interpreter::{Interpreter, Value}, OrionError, error, bug, Result, parser::Expr};
use std::collections::HashMap;

impl Interpreter {
    pub fn eval_lambda(
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
    pub fn eval_call(
        &mut self,
        function: &Box<Expr>,
        argument: &Box<Expr>,
        custom_scope: Option<&Vec<HashMap<String, Value>>>,
        ) -> Result<Value> {
        let arg = self.eval_expr(&**argument, custom_scope)?;

        let func = self.eval_expr(&**function, custom_scope)?;

        if let Value::Lambda(scopes, argument, body) = func.clone() {
            let mut new_scopes = scopes;
            new_scopes.push(HashMap::new());

            new_scopes.last_mut().unwrap().insert(argument, arg);

            if let Expr::Var(name) = &**function {
                new_scopes.last_mut().unwrap().insert(name.to_string(), func);
            }

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

}
