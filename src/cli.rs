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
use crate::{Result, print_err, error, lexer::{Lexer, Token}, parser::{Parser, Expr}, bytecode::Bytecode, compiler::Compiler, vm::{VM, Value}};

fn repl(dbg_level: u8, lib: String) -> Result<()> {
    println!(
        ";; Orion REPL v{}.\n
;; Copyright (C) 2021  Wafelack <wafelack@protonmail.com>
;; This program comes with ABSOLUTELY NO WARRANTY.
;; This is free software, and you are welcome to redistribute it
;; under certain conditions.",
env!("CARGO_PKG_VERSION")
);
    // let mut interpreter = Interpreter::new(vec![], no_prelude, quiet)?;
    let mut ctx = vec![];
    let mut sym_ref = vec![];
    let mut symbols = vec![];
    let mut bytecode = Bytecode::new();
    let mut constructors = vec![];
    let mut vm = VM::new(Bytecode::new());

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
                let tokens = match lex_dbg("REPL", i, line, dbg_level) {
                    Ok(t) => t,
                    Err(e) => {
                        print_err(e);
                        continue;
                    }
                };
                let expressions = match parse_dbg("REPL", tokens, dbg_level) {
                    Ok(e) => e,
                    Err(e) => {
                        print_err(e);
                        continue;
                    }
                };
                let (new_bytecode, new_syms, new_constructors) = match compile_dbg("REPL", expressions, dbg_level, symbols.clone(), bytecode.clone(), constructors.clone(), &lib, i > 1, true) {
                    Ok(b) => b,
                    Err(e) => {
                        print_err(e);
                        continue;
                    }
                };
                bytecode = new_bytecode;
                symbols = new_syms;
                constructors = new_constructors;
                let elapsed = start.elapsed();
                if dbg_level > 0 {
                    println!("{} Compiled in {}ms.", STAR, elapsed.as_millis());
                }

                match eval_dbg(&mut vm, &mut sym_ref, &mut ctx, bytecode.clone(), dbg_level) {
                    Ok(t) => if dbg_level > 0 {
                        println!("{} Run in {}ms.", STAR, t);
                    },
                    Err(e) => {
                        print_err(e);
                        continue;
                    }
                }
                println!("STACK: {:?}", vm.stack);
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

fn lex_dbg(file: impl ToString, line: usize, code: impl ToString, level: u8) -> Result<Vec<Token>> {
    if level > 0 {
        println!("{} Tokenizing {} SLOC...", STAR, code.to_string().lines().filter(|l| !l.is_empty()).count());
    }
    let tokens = Lexer::new(code, file).line(line).proc_tokens()?;
    if level > 1 {
        println!("[\x1b[0;33mTOKENS\x1b[0m]");
        tokens.iter().enumerate().for_each(|(idx, tok)| {
            println!("{:03}    {}:{:?}", idx, tok.line, tok.ttype);
        });
        println!("===========================================");
    } 
    Ok(tokens)
}
fn parse_dbg(file: impl ToString, code: Vec<Token>, level: u8) -> Result<Vec<Expr>> {
    if level > 0 {
        println!("{} Parsing {} Tokens...", STAR, code.len());
    }
    let expressions = Parser::new(code, file).parse()?;
    if level > 1 {
        println!("[\x1b[0;33mAST\x1b[0m]");
        expressions.iter().enumerate().for_each(|(idx, expr)| {
            println!("{:03}    {}:{:?}", idx, expr.line, expr.exprt);
        });
        println!("===========================================");
    } 
    Ok(expressions)
}
pub fn compile_dbg(file: impl ToString, expressions: Vec<Expr>, level: u8, symbols: Vec<(String, bool)>, bcode: Bytecode, constructors: Vec<String>, lib: impl ToString, already_loaded: bool, repl: bool) -> Result<(Bytecode, Vec<(String, bool)>, Vec<String>)> {
    if level > 0 {
        println!("{} Compiling {} Exprs...", STAR, expressions.len());
    }
    let (bytecode, symbols, constructors) = Compiler::new(expressions, file, bcode, constructors, already_loaded, lib.to_string(), repl)?.compile(symbols)?;
    if level > 1 {
        if bytecode.constants.len() != 0 {
            println!("[\x1b[0;33mBYTECODE.CONSTANTS\x1b[0m]");
            bytecode.constants.iter().enumerate().for_each(|(idx, constant)| println!("{:03}    {:?}", idx, constant));

        }
        if bytecode.symbols.len() != 0 {
            println!("[\x1b[0;33mBYTECODE.SYMBOLS\x1b[0m]");
            bytecode.symbols.iter().enumerate().for_each(|(idx, sym)| println!("{:03}    {}", idx, sym));
        }
        if bytecode.constructors.len() != 0 { 
            println!("[\x1b[0;33mBYTECODE.CONSTRUCTORS\x1b[0m]");
            bytecode.constructors.iter().enumerate().for_each(|(idx, containing)| println!("{:03}    {:?}", idx, containing));
        }
        if bytecode.chunks.len() != 0 {
            println!("[\x1b[0;33mBYTECODE.CHUNKS\x1b[0m]");
            bytecode.chunks.iter().enumerate().for_each(|(idx, chunk)| {
                println!("{:03} ::=", idx);
                println!("    [\x1b[0;34mREFERENCE\x1b[0m]");
                chunk.reference.iter().enumerate().for_each(|(idx, other_id)| println!("    {:03}    0x{:04x}", idx, other_id));
                println!("    [\x1b[0;34mINSTRUCTIONS\x1b[0m]");
                chunk.instructions.iter().enumerate().for_each(|(idx, instr)| println!("    {:03}    {:?}", idx, instr));
            });

        }
        if bytecode.patterns.len() != 0 {
            println!("[\x1b[0;33mBYTECODE.PATTERNS\x1b[0m]");
            bytecode.patterns.iter().enumerate().for_each(|(idx, pat)| {
                println!("{:03}    {:?}", idx, pat);
            })
        }
        if bytecode.matches.len() != 0 {
            println!("[\x1b[0;33mBYTECODE.MATCHES\x1b[0m]");
            bytecode.matches.iter().enumerate().for_each(|(idx, r#match)| {
                println!("{:03} ::= ", idx);
                r#match.into_iter().enumerate().for_each(|(idx, (pat_idx, instrs))| {
                    println!("    {:03} ::=  PAT_IDX = {}", idx, pat_idx);
                    println!("            [\x1b[0;33mPATTERN.INSTRUCTIONS\x1b[0m]");
                    instrs.into_iter().enumerate().for_each(|(idx, instr)| {
                        println!("            {:03}    {:?}", idx, instr);
                    })
                })
            })
        }
        if bytecode.instructions.len() != 0 {
            println!("[\x1b[0;33mBYTECODE.INSTRUCTIONS\x1b[0m]");
            bytecode.instructions.iter().enumerate().for_each(|(idx, instr)| println!("{:03}    {:?}", idx, instr));
            println!("===========================================");
        }
    } 
    Ok((bytecode, symbols, constructors))
}
fn eval_dbg(vm: &mut VM<256>, sym_ref: &mut Vec<u16>, ctx: &mut Vec<Rc<Value>>, bytecode: Bytecode, level: u8) -> Result<u64> {
    let mut stack = Vec::with_capacity(256);
    stack.push(Rc::new(Value::Tuple(vec![])));
    vm.input = bytecode;
    vm.stack = stack;
    vm.ip = 0;
    let start = Instant::now();
    let (new_ctx, new_ref) = vm.eval(sym_ref.clone(), ctx.clone())?;
    *ctx = new_ctx;
    *sym_ref = new_ref;
    let elapsed = start.elapsed();
    if level > 1 {
        if ctx.len() != 0 {
            println!("[\x1b[0;33mMEMORY\x1b[0m]");
            ctx.iter().enumerate().for_each(|(idx, v)| println!("{:03}   {:?}", idx, v));
        }
    }
    if level > 2 {
        println!("[\x1b[0;33mSTACK\x1b[0m]");
        vm.stack.iter().rev().enumerate().for_each(|(idx, v)| println!("{}    {:?}", if idx == 0 { "TS".to_string() } else { format!("{:03}", idx) }, v))
    }
    Ok(elapsed.as_millis() as u64)
}

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
        let tokens = lex_dbg(file, 1, content, dbg_level)?;
        let expressions = parse_dbg(file, tokens, dbg_level)?;
        let (bytecode, ..) = compile_dbg(file, expressions, dbg_level, vec![], Bytecode::new(), vec![], lib, false, false)?;
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
            let time = eval_dbg(&mut VM::<256>::new(Bytecode::new()), &mut vec![], &mut vec![], bytecode, dbg_level)?;
            if dbg_level > 0 {
                println!("{} Run in {}ms.", STAR, time)
            }
        }
    } else {
        repl(dbg_level, lib)?;
    }
    Ok(())
}
