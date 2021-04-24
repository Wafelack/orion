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
use crate::{interpreter::{Interpreter, Value}, OrionError, error, Result};
use std::{collections::HashMap, path::Path, fs::OpenOptions, io::{Write, self}};

impl Interpreter {
    pub fn put_str(&mut self, args: Vec<Value>, _: Option<&Vec<HashMap<String, Value>>>) -> Result<Value> {
        print!("{}", self.get_lit_val(&args[0]));
        Ok(Value::Unit)
    }
    pub fn put_str_ln(&mut self, args: Vec<Value>, _: Option<&Vec<HashMap<String, Value>>>) -> Result<Value> {
        println!("{}", self.get_lit_val(&args[0]));
        Ok(Value::Unit)
    }

    pub fn get_line(&mut self, _: Vec<Value>, _: Option<&Vec<HashMap<String, Value>>>) -> Result<Value> {
        let mut buffer = String::new();
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {}
            Err(e) => return error!("Failed to get line: {}.", e),
        }

        Ok(Value::String(buffer.trim().to_string()))

    }

    pub fn write(&mut self, args: Vec<Value>, _: Option<&Vec<HashMap<String, Value>>>) -> Result<Value> {
        let file = if let Value::String(of) = &args[1] {
            of
        } else {
            return error!("Expected a String, found a {}.", self.get_val_type(&args[1]));
        };

        if !Path::new(file).exists() {
            return error!("File `{}` does not exist.", file);
        }
        
        let written = match {match OpenOptions::new().append(true).open(file) {
            Ok(f) => f,
            Err(e) => return error!("Failed to open `{}`: {}.", file, e),
        }}.write(self.get_lit_val(&args[0]).as_bytes()) {
            Ok(u) => u,
            Err(e) => return error!("Failed to write `{}`: {}.", file, e),
        };


        Ok(Value::Integer(written as i32))
    }


}
