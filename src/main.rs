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
mod errors;
mod interpreter;
mod lexer;
mod parser;
mod tests;

use crate::{interpreter::Interpreter, lexer::Lexer, parser::Parser};
pub use errors::{OrionError, Result};
use std::{
    io::{self, Write},
    process::exit,
};

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

fn print_err(e: OrionError) {
    eprintln!(
        "[{}] {}",
        if cfg!(windows) {
            "Orion Error"
        } else {
            "\x1b[0;31mOrion Error\x1b[0m"
        },
        e.0
    );
}

fn try_main() -> Result<()> {
    let mut interpreter = Interpreter::new(vec![]);

    loop {
        let mut buffer = String::new();
        print!("Orion REPL> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {}
            Err(_) => {
                println!("Failed to read stream");
                continue;
            }
        }
        let tokens = match Lexer::new(&buffer).proc_tokens() {
            Ok(toks) => toks,
            Err(e) => {
                print_err(e);
                continue;
            }
        };
        let ast = match Parser::new(tokens).parse() {
            Ok(ast) => ast,
            Err(e) => {
                print_err(e);
                continue;
            }
        };

        interpreter.update_ast(ast);

        match interpreter.interpret(true) {
            Ok(_) => {}
            Err(e) => {
                print_err(e);
                continue;
            }
        }
        buffer.clear();
    }
}

fn main() {
    match try_main() {
        Ok(()) => {}
        Err(e) => {
            print_err(e);
            exit(1);
        }
    }
}
