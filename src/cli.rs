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
use clap::{App, Arg};
use rustyline::{error::ReadlineError, Editor};
use std::{rc::Rc, time::Instant, path::Path, fs, io::Write};
use crate::{Result, print_err, error, lexer::{Lexer, Token}, parser::{Parser, Expr}, bytecode::Bytecode, compiler::{Compiler, Macro}, vm::{VM, Value}};

fn repl(dbg_level: u8, lib: String) -> Result<()> {
    println!(
        ";; Orion REPL v{}.\n
;; Copyright (C) 2021  Wafelack <wafelack@protonmail.com>
;; This program comes with ABSOLUTELY NO WARRANTY.
;; This is free software, and you are welcome to redistribute it
;; under certain conditions.",
env!("CARGO_PKG_VERSION")
);
    let mut ctx = vec![];
    let mut symbols = vec![];
    let mut bytecode = Bytecode::new();
    let mut constructors = vec![];
    let mut sym_ref = vec![];
    let mut saves = vec![];
    let mut macros = vec![];
    let mut vm = VM::new(Bytecode::new(), vec![]);

    let mut rl = Editor::<()>::new();
    let mut i = 0;

    loop {
        i += 1;
        let line = rl.readline(&format!("orion:{:03}> ", i));

        match line {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line == "(quit)" {
                    return Ok(());
                }
                let start = Instant::now();
                let tokens = match Lexer::new(line, "REPL").line(i).proc_tokens() {
                    Ok(t) => t,
                    Err(e) => {
                        print_err(e);
                        continue;
                    }
                };
                    
                let expressions = match Parser::new(tokens, "REPL").parse() {
                   Ok(e) => e,
                   Err(e) => {
                       print_err(e);
                       continue;
                   }
                };
                let (new_bytecode, new_syms, new_constructors, new_macros) = match (match Compiler::new(expressions, "REPL", bytecode.clone(), constructors.clone(),  i > 1, lib.clone(), true, macros.clone()) {
                    Ok(c) => c,
                    Err(e) => {
                        if i == 1 {
                            i = 0;
                        }
                        print_err(e);
                        continue;
                    }
                }).compile(symbols.clone()) {
                    Ok(b) => b,
                    Err(e) => {
                        if i == 1 {
                            i = 0;
                        } 
                        print_err(e);
                        continue;
                    }
                };
                bytecode = new_bytecode;
                symbols = new_syms;
                constructors = new_constructors;
                macros = new_macros;
                let elapsed = start.elapsed();
                if dbg_level > 1 {
                    println!("{} Compiled in {}ms.", STAR, elapsed.as_millis());
                }
                vm = VM::<16000>::new(bytecode.clone(), saves.clone());
                let (new_ctx, new_ref, new_saves) = match vm.eval(sym_ref.clone(), ctx.clone(), dbg_level > 2) {
                    Ok(v) => v,
                    Err(e) => {
                        print_err(e);
                        continue;
                    }
                };
                ctx = new_ctx;
                sym_ref = new_ref;
                saves = new_saves;
                let top = &vm.stack.iter().nth(match vm.stack.len() as isize - 1 {
                    x if x < 0 => 0,
                    x => x as usize,
                }).and_then(|v| Some((**v).clone()));
                if let Some(Value::Tuple(v)) = top {
                    if !v.is_empty() {
                        println!("=> {}", vm.display_value(Rc::new(top.clone().unwrap()), true))
                    }
                } else if let Some(v) = top.clone() {
                    println!("=> {}", vm.display_value(Rc::new(v), true));
                }
 
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

const STAR: &str = "\x1b[0;32m*\x1b[0m";

macro_rules! get_app {
    ($name:literal, $version:expr) => {
        App::new($name)
            .version($version)
            .long_version(format!(
"{}
Copyright (C) 2021 Wafelack
License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.", $version).as_str())
            .about("LISP inspired purely functional programming language.")
            .after_help("Report bugs to: <https://github.com/orion-lang/orion/issues/new>\nOrion home page: <https://orion-lang.github.io>\nRepository: <https://github.com/orion-lang/orion>")
            .help_message("Print help information.")
            .version_message("Print version information.")
            .author(env!("CARGO_PKG_AUTHORS"))
            .arg(Arg::with_name("file")
                 .index(1)
                 .takes_value(true)
                 .value_name("FILE")
                 .help("The source file to compile."))
            .arg(Arg::with_name("lib")
                 .short("l")
                 .long("lib")
                 .takes_value(true)
                 .help("The library folder to use instead of $ORION_LIB."))
            .arg(Arg::with_name("compile-only")
                 .short("c")
                 .long("compile-only")
                 .help("Compile, but do not run."))
            .arg(Arg::with_name("output")
                 .short("o")
                 .long("output")
                 .takes_value(true)
                 .value_name("FILE")
                 .help("Place the output into FILE."))
            .arg(Arg::with_name("debug-level")
                 .short("d")
                 .long("debug")
                 .value_name("LEVEL")
                 .takes_value(true)
                 .help("Set the debug level. Defaults to 0."))
    }
}
use std::env;
pub fn cli() -> Result<()> {
    let matches = get_app!("Orion", env!("CARGO_PKG_VERSION")).get_matches();
    let lib = match matches.value_of("lib") {
        Some(l) => l.to_string(),
        None => match env::var("ORION_LIB") {
            Ok(v) => v.to_string(),
            Err(_) => return error!(=> "No such environment variable: ORION_LIB."),
        }
    };
    let dbg_level = match matches.value_of("debug-level") {
        Some(lvl) => match lvl.parse::<u8>() {
            Ok(u) => if u > 3 {
                3
            } else {
                u
            }
            Err(_) => 0,
        }
        None => 0,
    };
    if let Some(file) = matches.value_of("file") {
        let output = match matches.value_of("output") {
            Some(f) => f.to_string(),
            None => format!("{}.orc", Path::new(file).file_stem().unwrap().to_str().unwrap()),
        };
        let content = match fs::read_to_string(file) {
            Ok(s) => s,
            Err(e) => return error!(=> "Failed to read file: {}: {}.", file, e)
        };
        let start = Instant::now();
        let tokens = Lexer::new(content, file).proc_tokens()?;
        let expressions = Parser::new(tokens, file).parse()?;
        let (bytecode, ..) = Compiler::new(expressions, file, Bytecode::new(), vec![], false, lib, false, vec![])?.compile(vec![])?;
        let elapsed = start.elapsed();
        if dbg_level > 0 {
            println!("{} Compiled in {}ms.", STAR, elapsed.as_millis());
        }
        let to_write = bytecode.serialize();
        match (match fs::File::create(&output) {
            Ok(f) => f,
            Err(e) => return error!(=> "Failed to create file: {}: {}.", output, e)
        }).write_all(to_write.as_slice()) {
            Ok(()) => {}
            Err(e) => return error!(=> "Failed to write file: {}: {}.", output, e),
        };
        if !matches.is_present("compile-only") {
            VM::<16000>::new(bytecode, vec![]).eval(vec![], vec![], dbg_level > 2)?;
        }
    } else {
        repl(dbg_level, lib)?;
    }
    Ok(())
}
