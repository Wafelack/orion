/*
 *  Copyright (C) 2021, Wafelack <wafelack@protonmail.com>
 *
 *  ------------------------------------------------------
 *
 *     This file is part of Orion.
 *
 *  Orion is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Orion is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Orion.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::{vm::{VM, Value}, error, Result};
use std::rc::Rc;

impl<const STACK_SIZE: usize> VM<STACK_SIZE> {
    pub fn format(&mut self) -> Result<Rc<Value>> {
        let args = self.pop()?;
        let formatter = self.pop()?;
        if let Value::Tuple(args) = (*args).clone() {
            if let Value::String(formatter) = (*formatter).clone() {
                let fmt = "{}";
                let mut prev = 0;
                let to_ret = formatter.match_indices(fmt).enumerate().map(|(idx, (pos, _))| {
                    let to_ret = format!("{}{}", &formatter[prev..pos], &args[idx]);
                    prev = pos + fmt.len();
                    to_ret
                }).collect::<Vec<String>>().join("");

                Ok(Rc::new(Value::String(format!("{}{}", to_ret, &formatter[prev..]))))
            } else {
                error!(=> "Expected a String, found a {}.", self.val_type(&formatter)?)
            }
        } else {
            error!(=> "Expected a Tuple, found a {}.", self.val_type(&args)?)
        }
    }
}
