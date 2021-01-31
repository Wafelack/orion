use std::collections::BTreeMap;
use crate::interpreter::value::Value;
use crate::interpreter::interpreter::interpreter::Interpreter;
use std::path::Path;
use std::fs;

impl Interpreter {
    pub fn exists(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() < 1 {
            Ok(Value::Bool(false))
        } else {
            if let Value::String(s) = &args[0] {
                Ok(
                    Value::Bool(
                        Path::new(s).exists()
                    )
                )
            } else {
                Ok(
                    Value::Bool(false)
                )
            }
        }
    }
}