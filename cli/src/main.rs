use orion_lib::*;

fn main() -> orion_lib::Result<()> {
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
            Interpreter::new(ast).eval()?;
            Ok(())
        } else {
            Err(
                error!("File not found:", (args[0]))
            )
        }
    }
}

fn repl() -> orion_lib::Result<()> {
    let mut interpreter = Interpreter::new(
        Parser::new(
            Lexer::new(
                format!("(print \"Orion REPL v{}\")", env!("CARGO_PKG_VERSION"))
            ).scan_tokens()
        ).parse_tokens()?
    );
    interpreter.eval()?;

    use std::io::Write;

    loop {
        let mut line = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();
        if line.trim() == "(quit)" {
            return Ok(());
        }
        let mut lexer = Lexer::new(line.trim().to_owned());
        let toks = lexer.scan_tokens();
        let mut parser = Parser::new(toks);
        let ast = parser.parse_tokens()?;
        interpreter.process_ast(&ast)?;
    }
}
