use clap::{App, Arg};
use rustyline::{error::ReadlineError, Editor};
use crate::{Result, print_err, error, OrionError, lexer::Lexer, parser::Parser, compiler::Compiler, vm::VM};

fn repl(no_prelude: bool, debug: bool, quiet: bool) -> Result<()> {
    println!(
        ";; Orion REPL v{}.\n
;; Copyright (C) 2021  Wafelack <wafelack@protonmail.com>
;; This program comes with ABSOLUTELY NO WARRANTY.
;; This is free software, and you are welcome to redistribute it
;; under certain conditions.",
env!("CARGO_PKG_VERSION")
);
    // let mut interpreter = Interpreter::new(vec![], no_prelude, quiet)?;

    let mut rl = Editor::<()>::new();
    let mut i = 0;

    loop {
        i += 1;
        let line = rl.readline("orion> ");

        match line {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line == "(quit)" {
                    return Ok(());
                }

                let tokens = match Lexer::new(line.trim(), "REPL").line(i).proc_tokens() {
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

                let ast = match Parser::new(tokens, "REPL").parse() {
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

                let bytecode = Compiler::new(ast, "REPL").compile()?;
                println!("[INSTRUCTIONS]");
                bytecode
                    .instructions
                    .iter()
                    .for_each(|i| println!("{:?}", i));
                println!("\n[SYMBOLS]");
                bytecode
                    .symbols
                    .iter()
                    .enumerate()
                    .for_each(|(idx, var)| println!("0x{:04x}: {}", idx, var));
                println!("\n[CONSTANTS]");
                bytecode
                    .constants
                    .iter()
                    .enumerate()
                    .for_each(|(idx, constant)| println!("0x{:04x}: {:?}", idx, constant));
                println!("\n[CHUNKS]");
                bytecode.chunks.iter().enumerate().for_each(|(idx, chunk)| {
                    println!("{}: {{", idx);
                    println!("  reference: [");
                    chunk.reference.iter().for_each(|sym| {
                        println!("    0x{:04x}", sym);
                    });
                    println!("  ]\n  instructions: [");
                    chunk.instructions.iter().for_each(|instr| {
                        println!("    {:?}", instr);
                    });
                    println!("  ]\n}}");
                });

                let mut vm = VM::<256>::new(bytecode);
                println!("-------------------------------");
                println!("[STDOUT]");
                let memory = vm.eval()?;
                println!("\n[STACK]");
                vm.stack.into_iter().enumerate().for_each(|(idx, v)| {
                    println!("0x{:02x}: {:?}", idx, v);
                });
                println!("\n[MEMORY]");
                memory.into_iter().enumerate().for_each(|(idx, v)| {
                    println!("0x{:02x}: {:?}", idx, v);
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
pub fn cli() -> Result<()> {
    let matches = get_app!("Orion", env!("CARGO_PKG_VERSION")).get_matches();

    Ok(())
}
