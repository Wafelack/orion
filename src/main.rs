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
mod lambda;
mod patterns;
mod constructors;
mod builtins;

use crate::{interpreter::Interpreter, lexer::Lexer, parser::Parser};
pub use errors::{OrionError, Result};
use rustyline::{error::ReadlineError, Editor};
use clap::{App, Arg};
use std::{process::exit, path::Path, fs};

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
        "[{}] {}",
        if cfg!(windows) {
            "Orion Error"
        } else {
            "\x1b[0;31mOrion Error\x1b[0m"
        },
        e.0
        );
}

fn repl() {
    let mut interpreter = Interpreter::new(vec![]);

    println!(";; Welcome to Orion {}.\n
;; Orion REPL  Copyright (C) 2021  Wafelack <wafelack@protonmail.com>
;; This program comes with ABSOLUTELY NO WARRANTY.
;; This is free software, and you are welcome to redistribute it
;; under certain conditions.", env!("CARGO_PKG_VERSION"));

    let mut rl = Editor::<()>::new();

    loop {
        let line = rl.readline("> ");

        match line {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line == "(quit)" {
                    return;
                }

                let tokens = match Lexer::new(line.trim()).proc_tokens() {
                    Ok(t) => t,
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
                    Ok(_) => {},
                    Err(e) => {
                        print_err(e);
                        continue;
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!(";; User break");
            }
            Err(ReadlineError::Eof) => return,
            Err(_) => {
                eprintln!("An error occured, please retry.");
                continue;
            }

        }
    }
}

fn try_main() -> Result<()> {

    let matches = App::new("orion")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Orion is a purely functional lisp dialect.")
        .arg(Arg::with_name("file")
             .takes_value(true)
             .index(1)
             .help("The source file to pass to the interpreter"))
        .get_matches();

    if let Some(path) = matches.value_of("file") {
        if Path::new(path).exists() {

            let content = match fs::read_to_string(path) {
                Ok(c) => c,
                Err(e) => return error!("fatal: Failed to read file: {}.", e),
            };

            let tokens = Lexer::new(content).proc_tokens()?;
            let ast = Parser::new(tokens).parse()?;
            Interpreter::new(ast).interpret(false)?;
            Ok(())
        } else {
            error!("fatal: File not found: {}.", path)
        }
    } else {
        repl();
        Ok(())
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
