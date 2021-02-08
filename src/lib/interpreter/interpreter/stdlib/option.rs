use crate::interpreter::value::Value;
use crate::interpreter::interpreter::interpreter::Interpreter;
use crate::error;

impl Interpreter {
    pub fn some(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        
        if args.len() != 1 {
            return Err(
                error!("Invalid number of arguments, expected 1, found", (args.len()))
            );
        }

        Ok(
            Value::Option(
                Some(
                    Box::new(args[0].clone())
                )
            )
        )
    }
    pub fn none(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 0 {
            return Err(
                crate::error!("Invalid number of arguments, expected 0, found", (args.len()))
            );
        }

        Ok(
            Value::Option(
                None
            )
        )
    }
    pub fn unwrap(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 1 {
            return Err(
                crate::error!("Invalid number of arguments, expected 1, found", (args.len()))
            );
        }

        if let Value::Option(opt) = &args[0] {
            if opt.is_some() {
                Ok(
                    *(opt.clone().unwrap())
                )
            } else {
                Err(
                    error!("Attempted to call `unwrap` on a None value.")
                )
            }
        } else {
            Err(
                error!("Invalid argument, expected option, found", (&args[0].get_type()))
            )
        }
    }
    pub fn unwrap_or(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 2 {
            return Err(
                crate::error!("Invalid number of arguments, expected 2, found", (args.len()))
            );
        }

        if let Value::Option(opt) = &args[0] {
            if opt.is_some() {
                Ok(
                    *(opt.clone().unwrap())
                )
            } else {
                Ok(
                    args[1].to_owned()
                )
            }
        } else {
            Err(
                error!("Invalid argument, expected option, found", (&args[0].get_type()))
            )
        }
    }
}