mod lib;

use lib::*;

fn main() -> lib::Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    if args.len() < 1 {
        repl()
    } else {
        use std::path::Path;

        if Path::new(&args[0]).exists() {
            let code = std::fs::read_to_string(&args[0]).map_err(|e| error!(e))?;
            let toks = Lexer::new(code).scan_tokens();
            let ast = Parser::new(toks).parse_tokens()?;
            if args.contains(&"--debug".to_owned()) {
                println!("{}", ast);
            }
            Interpreter::new(ast, args.iter().filter(|x| x != &&"--debug").map(|x| x.to_owned()).collect::<Vec<String>>()).eval()?;
            Ok(())
        } else {
            Err(
                error!("File not found:", (args[0]))
            )
        }
    }
}

fn repl() -> lib::Result<()> {
    use rustyline::error::ReadlineError;
    use rustyline::Editor;

    let mut interpreter = Interpreter::new(
        Parser::new(
            Lexer::new(
                format!("(print \"Orion REPL v{}\")", env!("CARGO_PKG_VERSION"))
            ).scan_tokens()
        ).parse_tokens()?, vec![],
    );
    interpreter.eval()?;

    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line == "(quit)" {
                    return Ok(());
                }
                let mut lexer = Lexer::new(line.trim().to_owned());
                let toks = lexer.scan_tokens();
                let mut parser = Parser::new(toks);
                let ast = match parser.parse_tokens() {
                    Ok(n) => n,
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };
                match interpreter.process_ast(&ast) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };
            },
            Err(ReadlineError::Interrupted) => {
                println!("Keyboard Interrupt");
            },
            Err(ReadlineError::Eof) => {
                println!("(quit)");
                return Ok(())
            },
            Err(err) => {
                return Err(
                    error!(err)
                );
            }
        }
    }
}
