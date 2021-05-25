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
mod bytecode;
mod compiler;
mod errors;
mod lexer;
mod parser;
mod vm;
mod cli;

mod arithmetic;
mod io;
mod string;

use crate::cli::cli;
pub use errors::{OrionError, Result};
use std::process::exit;

#[macro_export]
macro_rules! bug {
    ($bug:literal) => {
        panic!(
            "This is a bug, please report it with the following information: {}: [{}:{}]",
            $bug,
            file!(),
            line!()
            )
    };
}
#[macro_export]
macro_rules! table {
    {$($key:expr => $value:expr),+} => {
                                           {
                                               let mut map = HashMap::new();

                                               $(
                                                   map.insert($key, $value);
                                                )*

                                                   map
                                           }
                                       };
}
fn print_err(e: OrionError) {
    eprintln!(
        "{}{}",
        if e.0.is_some() && e.1.is_some() {
            format!("{}:{}: ", e.0.unwrap(), e.1.unwrap())
        } else {
            if cfg!(windows) {
                "Error: "
            } else {
                "\x1b[0;31mError: \x1b[0m"
            }.to_string()
        },
        e.2);
}
fn main() {
    match cli() {
        Ok(()) => {}
        Err(e) => {
            print_err(e);
            exit(1);
        }
    }
}
