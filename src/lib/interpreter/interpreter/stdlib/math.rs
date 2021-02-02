use crate::interpreter::value::Value;
use crate::interpreter::interpreter::interpreter::Interpreter;
use std::path::Path;
use std::io::Write;
use std::fs;
use crate::*;
use std::fs::File;

impl Interpreter {
    pub fn cos(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 1 {
            return Err(
                crate::error!("Invalid number of arguments, expected 1, found", (args.len()))
            );
        }


        if let Value::Float(f) = &args[0] {
            Ok(Value::Float(
                f.cos()
            ))
        } else {
            Err(
                error!("Invalid argument, expected float, found", (&args[0].get_type()))
            )
        }
    }
    pub fn sin(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 1 {
            return Err(
                crate::error!("Invalid number of arguments, expected 1, found", (args.len()))
            );
        }


        if let Value::Float(f) = &args[0] {
            Ok(Value::Float(
                f.sin()
            ))
        } else {
            Err(
                error!("Invalid argument, expected float, found", (&args[0].get_type()))
            )
        }
    }

    pub fn tan(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 1 {
            return Err(
                crate::error!("Invalid number of arguments, expected 1, found", (args.len()))
            );
        }


        if let Value::Float(f) = &args[0] {
            Ok(Value::Float(
                f.tan()
            ))
        } else {
            Err(
                error!("Invalid argument, expected float, found", (&args[0].get_type()))
            )
        }
    }


    pub fn acos(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 1 {
            return Err(
                crate::error!("Invalid number of arguments, expected 1, found", (args.len()))
            );
        }


        if let Value::Float(f) = &args[0] {
            Ok(Value::Float(
                f.acos()
            ))
        } else {
            Err(
                error!("Invalid argument, expected float, found", (&args[0].get_type()))
            )
        }
    }
    pub fn asin(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 1 {
            return Err(
                crate::error!("Invalid number of arguments, expected 1, found", (args.len()))
            );
        }


        if let Value::Float(f) = &args[0] {
            Ok(Value::Float(
                f.asin()
            ))
        } else {
            Err(
                error!("Invalid argument, expected float, found", (&args[0].get_type()))
            )
        }
    }

    pub fn atan(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 1 {
            return Err(
                crate::error!("Invalid number of arguments, expected 1, found", (args.len()))
            );
        }


        if let Value::Float(f) = &args[0] {
            Ok(Value::Float(
                f.atan()
            ))
        } else {
            Err(
                error!("Invalid argument, expected float, found", (&args[0].get_type()))
            )
        }
    }
}