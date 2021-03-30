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

        println!("{:?}", ast);

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
