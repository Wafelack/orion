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
use crate::{interpreter::{Interpreter, Value}, OrionError, error, bug, Result, parser::{Pattern, Expr, Literal}};
use std::collections::HashMap;

impl Interpreter {
    pub fn format(&mut self, args: Vec<Value>) -> Result<Value> {
        let base_str = if let Value::String(s) = &args[0] {
            s
        } else {
            return error!("Expected a String, found a {}.", self.get_val_type(&args[0]));
        };

        let formatter = "#v";

        if base_str.matches(formatter).count() != args.len() - 1 {
            return error!("Expected {} arguments, found {}.", base_str.matches(formatter).count() + 1, args.len());
        }

        let mut toret = String::new();
        let mut prev_pos = 0;
        base_str.match_indices(formatter).enumerate().for_each(|(idx, (pos, _))| {
            toret.push_str(&base_str[prev_pos..pos]);
            toret.push_str(&self.get_lit_val(&args[idx + 1]));
            prev_pos = pos + formatter.len();
        });
        toret.push_str(&base_str[prev_pos..]);
        
        Ok(Value::String(toret))
    }
}
