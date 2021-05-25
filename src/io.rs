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
use std::io::{self, Write};

impl <const STACK_SIZE: usize> VM<STACK_SIZE> {
    pub fn put_str(&mut self) -> Result<Value> {
        let to_print = self.pop()?;

        match to_print {
            Value::String(s) => {
                print!("{}", s);
                io::stdout().flush().unwrap();
                Ok(Value::Tuple(vec![]))
            },
            _ => error!(=> "Expected a String, found a {:?}.", to_print)
        }
    }
    pub fn get_line(&mut self) -> Result<Value> {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => Ok(Value::String(buffer.trim().to_string())),
            Err(_) => error!(=> "Failed to get line from user input."),
        }
    }
}
