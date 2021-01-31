use crate::interpreter::value::Value;
use crate::interpreter::interpreter::interpreter::Interpreter;

impl Interpreter {
    pub fn print(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        for arg in args {
            print!("{}", arg);
        }
        println!();
        Ok(Value::Nil)
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