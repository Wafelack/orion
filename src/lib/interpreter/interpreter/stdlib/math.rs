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
                f.to_radians().cos()
            ))
        } else {
            Err(
                error!("Invalid argument, expected float, found", (&args[0].get_type()))
            )
        }
    }
    pub fn sqrt(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 1 {
            return Err(
                crate::error!("Invalid number of arguments, expected 1, found", (args.len()))
            );
        }


        if let Value::Float(f) = &args[0] {
            Ok(Value::Float(
                f.sqrt()
            ))
        } else {
            Err(
                error!("Invalid argument, expected float, found", (&args[0].get_type()))
            )
        }
    }
    pub fn pow(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 2 {
            return Err(
                crate::error!("Invalid number of arguments, expected 1, found", (args.len()))
            );
        }


        if let Value::Float(f) = &args[0] {
            if let Value::Int(pow) = &args[1] {
                Ok(
                    Value::Float(f.powi(*pow))
                )
            } else {
                Err(
                    error!("Invalid argument, expected int, found", (&args[1].get_type()))
                )
            }
        } else if let Value::Int(i) = &args[0] {
            if let Value::Int(pow) = &args[1] {
                if *pow <= 0 {
                    return Err(
                        error!("Invalid argument, expected positive int")
                    )
                }
                Ok(
                    Value::Int(i.pow(*pow as u32))
                )
            } else {
                Err(
                    error!("Invalid argument, expected int, found", (&args[1].get_type()))
                )
            }
        }  else {
            Err(
                error!("Invalid argument, expected float or int, found", (&args[0].get_type()))
            )
        }
    }

    pub fn max(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 2 {
            return Err(
                crate::error!("Invalid number of arguments, expected 1, found", (args.len()))
            );
        }


        if let Value::Float(a) = &args[0] {
            if let Value::Float(b) = &args[1] {
                Ok(
                    Value::Float(a.max(*b))
                )
            } else {
                Err(
                    error!("Invalid argument, expected int, found", (&args[1].get_type()))
                )
            }
        } else {
            Err(
                error!("Invalid argument, expected float, found", (&args[0].get_type()))
            )
        }
    }

    pub fn min(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 2 {
            return Err(
                crate::error!("Invalid number of arguments, expected 1, found", (args.len()))
            );
        }


        if let Value::Float(a) = &args[0] {
            if let Value::Float(b) = &args[1] {
                Ok(
                    Value::Float(a.min(*b))
                )
            } else {
                Err(
                    error!("Invalid argument, expected int, found", (&args[1].get_type()))
                )
            }
        } else {
            Err(
                error!("Invalid argument, expected float, found", (&args[0].get_type()))
            )
        }
    }

    pub fn clamp(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 3 {
            return Err(
                crate::error!("Invalid number of arguments, expected 3, found", (args.len()))
            );
        }


        if let Value::Float(n) = &args[0] {
            if let Value::Float(start) = &args[1] {
                if let Value::Float(end) = &args[2] {
                    if *n < *start {
                        Ok(
                            Value::Float(*start)
                        )
                    } else if *n > *end {
                        Ok(
                            Value::Float(*end)
                        )
                    } else {
                        Ok(
                            Value::Float(*n)
                        )
                    }
                } else {
                    Err(
                        error!("Invalid argument, expected float, found", (&args[2].get_type()))
                    )
                }
            } else {
                Err(
                    error!("Invalid argument, expected float, found", (&args[1].get_type()))
                )
            }
        } else {
            Err(
                error!("Invalid argument, expected float, found", (&args[0].get_type()))
            )
        }
    }

    pub fn range(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 2 {
            return Err(
                crate::error!("Invalid number of arguments, expected 1, found", (args.len()))
            );
        }


        if let Value::Int(a) = &args[0] {
            if let Value::Int(b) = &args[1] {
                let mut toret = vec![];
                for i in *a..*b {
                    toret.push(Value::Int(i));
                }
                Ok(
                    Value::List(
                        toret
                    )
                )
            } else {
                Err(
                    error!("Invalid argument, expected int, found", (&args[1].get_type()))
                )
            }
        } else {
            Err(
                error!("Invalid argument, expected float or int, found", (&args[0].get_type()))
            )
        }
    }

    pub fn odd(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 1 {
            return Err(
                crate::error!("Invalid number of arguments, expected 1, found", (args.len()))
            );
        }


        if let Value::Int(i) = &args[0] {
            Ok(Value::Bool(
                *i % 2 != 0
            ))
        }  else {
            Err(
                error!("Invalid argument, expected int, found", (&args[0].get_type()))
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
                (0.0174533 * f).sin()
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
                (0.0174533 * f).tan()
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
                f.acos() / 0.0174533
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
                f.asin() / 0.0174533
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
                f.atan() / 0.0174533
            ))
        } else {
            Err(
                error!("Invalid argument, expected float, found", (&args[0].get_type()))
            )
        }
    }
}