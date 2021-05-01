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
mod lexer;
mod parser;
mod bytecode;
mod compiler;

// mod builtins;

use crate::{lexer::Lexer, parser::Parser, compiler::Compiler};
pub use errors::{OrionError, Result};
use rustyline::{error::ReadlineError, Editor};
use clap::{App, Arg};
use std::{time::Instant, process::exit, path::Path, fs};

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
        if cfg!(windows) {
            "Error: "
        } else {
            "\x1b[0;31mError: \x1b[0m"
        },
        e.0
        );
}

fn repl(no_prelude: bool, debug: bool, quiet: bool) -> Result<()> {

    println!(";; Welcome to Orion {}.\n
;; Orion REPL  Copyright (C) 2021  Wafelack <wafelack@protonmail.com>
;; This program comes with ABSOLUTELY NO WARRANTY.
;; This is free software, and you are welcome to redistribute it
;; under certain conditions.", env!("CARGO_PKG_VERSION"));
    // let mut interpreter = Interpreter::new(vec![], no_prelude, quiet)?;

    let mut rl = Editor::<()>::new();

    loop {
        let line = rl.readline("> ");

        match line {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line == "(quit)" {
                    return Ok(());
                }

                let tokens = match Lexer::new(line.trim()).proc_tokens() {
                    Ok(t) => t,
                    Err(e) => {
                        print_err(e);
                        continue;
                    }
                };

                if debug {
                    println!("Tokens\n======");
                    tokens.iter().for_each(|t| {
                        println!("{}", t.display());
                    });
                    println!();
                }

                let ast = match Parser::new(tokens).parse() {
                    Ok(ast) => ast,
                    Err(e) => {
                        print_err(e);
                        continue;
                    }
                };


                if debug {
                    println!("Syntax Tree\n===========");
                    ast.iter().for_each(|e| println!("{}", e.get_type()));
                }

                // interpreter.update_ast(ast);

                if debug {
                    println!("\nStdout\n======");
                }
                
                let bytecode = Compiler::new(ast).compile()?;
                println!("[INSTRUCTIONS]");
                bytecode.instructions.into_iter().for_each(|i| println!("{:?}", i));
                println!("\n[SYMBOLS]");
                bytecode.symbols.into_iter().enumerate().for_each(|(idx, var)| println!("{}: {}", idx, var));
                println!("\n[CONSTANTS]");
                bytecode.constants.into_iter().enumerate().for_each(|(idx, constant)| println!("{}: {:?}", idx, constant)); 
                println!("\n[CHUNKS]");
                bytecode.chunks.into_iter().enumerate().for_each(|(idx, chunk)| {
                    println!("{}: {{", idx);
                    chunk.into_iter().enumerate().for_each(|(id, instr)| {
                        println!("\t{}: {:?}", id, instr);
                    });
                    println!("}}");
                });

                /* 
                let start = Instant::now();
                /* match interpreter.interpret(true) {
                    Ok(_) => {},
                    Err(e) => {
                        print_err(e);
                        continue;
                    }
                } */
                let elapsed = start.elapsed();
                if debug {
                    println!("\nDone in {}ms.", elapsed.as_millis());
                } */
            }
            Err(ReadlineError::Interrupted) => {
                println!(";; User break");
            }
            Err(ReadlineError::Eof) => return Ok(()),
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
        .arg(Arg::with_name("no-load-prelude")
             .short("np")
             .long("no-load-prelude")
             .help("Do not load the prelude file"))
        .arg(Arg::with_name("debug")
             .short("d")
             .long("debug")
             .help("Display debug information."))
        .arg(Arg::with_name("quiet")
             .short("q")
             .long("quiet")
             .help("Do not display messages."))
        .get_matches();

    if let Some(path) = matches.value_of("file") {
        if Path::new(path).exists() {

            let content = match fs::read_to_string(path) {
                Ok(c) => c,
                Err(e) => return error!("fatal: Failed to read file: {}.", e),
            };

            let tokens = Lexer::new(content).proc_tokens()?;
            if matches.is_present("debug") {
                println!("Tokens\n======");
                tokens.iter().for_each(|t| {
                    println!("{}", t.display());
                });
                println!();
            }

            let ast = Parser::new(tokens).parse()?;

            if matches.is_present("debug") {
                println!("Syntax Tree\n===========");
                ast.iter().for_each(|e| println!("{}", e.get_type()));
            }
            if matches.is_present("debug") {
                println!("\nStdout\n======");
            }
            let start = Instant::now();
            // Interpreter::new(ast, matches.is_present("no-load-prelude"), matches.is_present("quiet"))?.interpret(false)?;
            let elapsed = start.elapsed();
            if matches.is_present("debug") {
                println!("\nDone in {}ms.", elapsed.as_millis());
            }
            Ok(())
        } else {
            error!("fatal: File not found: {}.", path)
        }
    } else {
        repl(matches.is_present("no-load-prelude"), matches.is_present("debug"), matches.is_present("quiet"))?;
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
