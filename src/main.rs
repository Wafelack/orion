mod errors;
// mod interpreter;
mod lexer;
mod parser;
mod tests;

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

fn try_main() -> Result<()> {
    Ok(())
}

fn main() {
    match try_main() {
        Ok(()) => {}
        Err(e) => {
            eprintln!(
                "[{}] {}",
                if cfg!(windows) {
                    "Orion Error"
                } else {
                    "\x1b[0;31mOrion Error\x1b[0m"
                },
                e.0
            );
            exit(1);
        }
    }
}
