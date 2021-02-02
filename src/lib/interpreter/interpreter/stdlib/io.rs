use crate::interpreter::value::Value;
use crate::interpreter::interpreter::interpreter::Interpreter;
use std::io;

impl Interpreter {
    pub fn print(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        for arg in args {
            print!("{}", arg);
        }
        println!();
        Ok(Value::Nil)
    }

    pub fn input(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        use std::io::Write;

        Ok(if args.len() < 1 {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            Value::String(
                line
            )
        } else {
            let mut line = String::new();
            print!("{}", args[0]);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut line).unwrap();
            Value::String(
                line
            )
        })
    }

    pub fn puts(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        for arg in args {
            print!("{}", arg);
        }
        Ok(Value::Nil)
    }

    pub fn eprint(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        for arg in args {
            eprint!("{}", arg);
        }
        eprintln!();
        Ok(Value::Nil)
    }
    pub fn eputs(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        for arg in args {
            eprint!("{}", arg);
        }
        Ok(Value::Nil)
    }
}