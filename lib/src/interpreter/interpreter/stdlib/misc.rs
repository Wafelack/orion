use crate::interpreter::value::Value;
use crate::interpreter::interpreter::interpreter::Interpreter;

impl Interpreter {
    pub fn assert(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 1 {
            return Err(
                error!("Invalid number of arguments, expected 1, found", (args.len()))
            );
        }

        if let Value::Bool(b) = &args[0] {
            if *b {
                return Ok(Value::Nil);
            } else {
                panic!("Assertion failed.")
            }
        } else {
            panic!("Assertion failed.")
        }
    }

    pub fn import(&mut self, args: &Vec<Value>) -> crate::Result<Value> {

        use std::path::Path;

        for arg in args {
            if let Value::String(s) = arg {
                if !Path::new(&s).exists() {
                    return Err(
                        error!("Cannot find file `", s, "`.")
                    )
                }

                let content = match std::fs::read_to_string(&s) {
                    Ok(c) => c,
                    Err(e) => return Err(error!(e)),
                };

                let mut lexer = crate::lexer::lexer::Lexer::new(content);
                let tokens = lexer.scan_tokens();
                let mut parser = crate::parser::parser::Parser::new(tokens);
                let ast = parser.parse_tokens()?;
                self.eval_calls(&ast.children)?; // Delete the Scope.
            } else {
                return Err(
                    error!("Invalid argument, expected string,  found", (arg.get_type()))
                )
            }
        }

        Ok(Value::Nil)
    }
}