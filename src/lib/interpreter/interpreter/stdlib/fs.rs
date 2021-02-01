use crate::interpreter::value::Value;
use crate::interpreter::interpreter::interpreter::Interpreter;
use std::path::Path;
use std::fs;
use crate::*;

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
    pub fn read_dir(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 1 {
            return Err(
                error!("Invalid number of arguments, expected 1, found", (args.len()))
            )
        }

        if let Value::String(s) = &args[0] {
            if Path::new(s).exists() {
                Ok(
                    Value::List(see_dir(s)?)
                )

            } else {
                Ok(
                    Value::List(vec![])
                )
            }
        } else {
            return Err(
                error!("Invalid argument, expected string,  found", (args[0].get_type()))
            )
        }
    }
}

fn see_dir(path: &str) -> crate::Result<Vec<Value>> {
    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(e) => return Err(error!(e)),
    };
    let mut toret = vec![];
    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => return Err(error!(e)),
        };

        if entry.path().is_dir() {
            toret.push(Value::String(entry.path().to_str().unwrap().to_owned()));
            toret.extend(see_dir(entry.path().to_str().unwrap())?);
        } else {
            toret.push(Value::String(entry.path().to_str().unwrap().to_owned()));
        }
    }

    Ok(toret)
}