use std::collections::BTreeMap;
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
    pub fn list(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        
        let mut first = "nil".to_owned();
        let mut toret = vec![];
        for i in 0..args.len() {
            if i == 0 {
                first = args[i].get_type();
            }

            if args[i].get_type() != first {
                return Err(
                    error!("Invalid argument, expected", first, "found", (args[i].get_type()))
                )
            }


            toret.push(args[i].clone());
        }

        Ok(Value::List(toret))
    }
    pub fn index(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 2 {
            return Err(
                error!("Invalid number of arguments, expected 2, found", (args.len()))
            )
        }

        let index = if let Value::Int(i) = &args[1] {
            if *i > 0 {
                *i as usize
            } else {                
                return Err(
                    error!("Invalid argument, expected integer between 0 and", (std::usize::MAX),"found", i)
                )
            }
        } else {
            return Err(
                error!("Invalid argument, expected integer, found", (&args[1].get_type()))
            )
        };

        let toret = if let Value::String(s) = &args[0] {
            if index >= s.len() {
                return Err(
                    error!("Index out of bounds, the length is", (s.len()), "but the index is", index)
                )
            }
            Value::String(
                s[index..index + 1].to_owned()
            )
        } else if let Value::List(l) = &args[0] {
            if index >= l.len() {
                return Err(
                    error!("Index out of bounds, the length is", (l.len()), "but the index is", index)
                )
            }
            l[index].clone()
        } else {
            return Err(
                error!("Invalid argument, expected string or list, found", (&args[1].get_type()))
            )
        };

        Ok(toret)
    }
}