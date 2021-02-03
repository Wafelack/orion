use crate::interpreter::value::Value;
use crate::interpreter::interpreter::interpreter::Interpreter;
use crate::*;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

pub struct MtGenerator {
    seed: f64,
    j: f64,
    k: f64,
    period: u64,
}

impl MtGenerator {
    pub fn new(seed: i32) -> Self {
        Self {
            seed: seed as f64,
            j: 2f64.powi(31) - 1.,
            k: 16807.,
            period: 2u64.pow(30),
        }
    }
    pub fn gen_number(&mut self, min: i32, max: i32) -> i32 {
        self.seed = (self.k * self.seed) % self.j;
        let toret = (max as f64 - min as f64 + 1.) * (self.seed / self.j) + min as f64;
        self.period -= 1;
        if self.period == 0 {
            self.period = 2u64.pow(30)
        }
        toret.ceil() as i32
    }
}

impl Interpreter {
    pub fn init_rand(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() > 1 {
            return Err(
                crate::error!("Invalid number of arguments, expected 1|0, found", (args.len()))
            );
        }

        if args.len() == 1 {
            if let Value::Int(i) = &args[0] {
                self.rng = Some(MtGenerator::new(*i));
            } else {
                return Err(
                    error!("Invalid argument, expected int, found", (&args[0].get_type()))
                )
            }
        } else {
            let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            self.rng = Some(MtGenerator::new(
                if seed > std::i32::MAX as u64{
                    std::i32::MAX
                } else {
                    seed as i32
                }
            ));
        }

        Ok(Value::Nil)
    }
    pub fn gen_rand(&mut self, args: &Vec<Value>) -> crate::Result<Value> {

        if self.rng.is_none() {
            return Err(
                crate::error!("Error, random number generator is not initialized, initialize it with `math:initRng [seed]`")
            );
        }

        if args.len() != 2 {
            return Err(
                crate::error!("Invalid number of arguments, expected 2, found", (args.len()))
            );
        }

        if let Value::Int(start) = &args[0] {
            if let Value::Int(end) = &args[1] {
                Ok(
                    Value::Int(
                        self.rng.as_mut().unwrap().gen_number(*start, *end)
                    )
                )
            } else {
                Err(
                    error!("Invalid argument, expected int, found", (&args[1].get_type()))
                )
            }
        } else {
            Err(
                error!("Invalid argument, expected int, found", (&args[0].get_type()))
            )
        }

    }
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
                crate::error!("Invalid number of arguments, expected 2, found", (args.len()))
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