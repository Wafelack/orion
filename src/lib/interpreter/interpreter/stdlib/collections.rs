use std::collections::BTreeMap;
use crate::interpreter::value::Value;
use crate::interpreter::interpreter::interpreter::Interpreter;
use crate::*;
use std::io;

impl Interpreter {
    pub fn list(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        
        let mut first = "nil".to_owned();
        let mut toret = vec![];
        for i in 0..args.len() {
            if i == 0 {
                first = args[0].get_type();
            }

            if args[i].get_type() != first {
                return Err(
                    crate::error!("Invalid argument, expected", first, "found", (args[i].get_type()))
                )
            }


            toret.push(args[i].clone());
        }

        Ok(Value::List(toret))
    }
    pub fn slice(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 3 {
            return Err(
                crate::error!("Invalid number of arguments, expected 3, found", (args.len()))
            )
        }

        if let Value::Int(start) = &args[1]  {
            if let Value::Int(end) = &args[2] {
                if let Value::String(s) = &args[0] {
                    let i_start = if *start >= 0 {
                        *start as usize
                    } else {
                        return Err(
                            crate::error!("Invalid argument, expected integer between 0 and", (std::usize::MAX),"found", start)
                        )
                    };

                    let i_end = if *end >= 0 {
                        *end as usize
                    } else {
                        return Err(
                            crate::error!("Invalid argument, expected integer between 0 and", (std::usize::MAX),"found", end)
                        )
                    };

                    if i_start >= s.len() || i_end >= s.len() {
                        return Err(
                            crate::error!("Index out of bounds, the length is", (s.len()), "but the index is", (i_start.max(i_end)))
                        )
                    } else {
                        return Ok(
                            Value::String(
                                s[i_start..i_end].to_owned()
                            )
                        )
                    }

                } else if let Value::List(l) = &args[0] {
                    let i_start = if *start >= 0 {
                        *start as usize
                    } else {
                        return Err(
                            crate::error!("Invalid argument, expected integer between 0 and", (std::usize::MAX),"found", start)
                        )
                    };

                    let i_end = if *end >= 0 {
                        *end as usize
                    } else {
                        return Err(
                            crate::error!("Invalid argument, expected integer between 0 and", (std::usize::MAX),"found",start )
                        )
                    };

                    if i_start >= l.len() || i_end > l.len() {
                        return Err(
                            crate::error!("Index out of bounds, the length is", (l.len()), "but the index is", (i_start.max(i_end)))
                        )
                    } else {
                        let toret = Value::List(
                            l[i_start..i_end].to_vec()
                        );
                        return Ok(
                            toret
                        )
                    }
                } else {
                    Err(
                        crate::error!("Invalid argument, expected list or string, found", (&args[0].get_type()))
                    )
                }
            } else {
                Err(
                    crate::error!("Invalid argument, expected int, found", (&args[2].get_type()))
                )
            }
        } else {
            Err(
                crate::error!("Invalid argument, expected int, found", (&args[1].get_type()))
            )
        }
    }



    pub fn object(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        
        if args.len() % 2 != 0 {
            return Err(
                crate::error!("Invalid arguments, expected an even number of arguments, found", (args.len()))
            )
        }

        let mut toret: BTreeMap<String, Value> = BTreeMap::new();

        let mut i = 0;

        while i < args.len() {
            if let Value::String(s) = &args[i] {
                toret.insert(
                    s.to_owned(),
                    args[i + 1].to_owned(),
                );
                i += 2;
            } else {
                return Err(
                    crate::error!("Invalid argument, expected string, found", (args[i].get_type()))
                )
            }
        }

        Ok(
            Value::Object(toret)
        )
    }

    pub fn index(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 2 {
            return Err(
                crate::error!("Invalid number of arguments, expected 2, found", (args.len()))
            )
        }

        if let Value::Int(i) = &args[1] {
            let index = if *i >= 0 {
                *i as usize
            } else {                
                return Err(
                    crate::error!("Invalid argument, expected integer between 0 and", (std::usize::MAX),"found", i)
                )
            };

            let toret = if let Value::String(s) = &args[0] {
                if index >= s.len() {
                    return Err(
                        crate::error!("Index out of bounds, the length is", (s.len()), "but the index is", index)
                    )
                }
                Value::String(
                    s[index..index + 1].to_owned()
                )
            } else if let Value::List(l) = &args[0] {
                if index >= l.len() {
                    return Err(
                        crate::error!("Index out of bounds, the length is", (l.len()), "but the index is", index)
                    )
                }
                l[index].clone()
            } else {
                return Err(
                    crate::error!("Invalid argument, expected string or list, found", (&args[1].get_type()))
                )
            };

            Ok(toret)

        } else if let Value::String(s) = &args[1] {

            let toret = if let Value::Object(map) = &args[0] {
                if map.contains_key(s) {
                    map[s].clone()
                } else {
                    return Err(
                        crate::error!("Key `", s, "` not found in object `", (&args[0]),"`.")
                    )
                }
            } else {
                return Err(
                    crate::error!("Invalid argument, expected object, found", (&args[1].get_type()))
                )
            };

            Ok(toret)
        } else {
            return Err(
                crate::error!("Invalid argument, expected integer, found", (&args[1].get_type()))
            )
        }   
    }

    pub fn push(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 2  && args.len() != 3{
                return Err(
                    crate::error!("Invalid number of arguments, expected 2|3, found", (args.len()))
                )
        }

        if let Value::List(mut l) = args[0].clone() {
            if let Some(el) = l.last() {
                if args[1].get_type() != el.get_type() {
                    Err(
                        crate::error!("Type error: list is of type", (el.get_type()), "but element is", (args[1].get_type()),".")
                    )
                } else {
                    l.push(args[1].to_owned());
                    Ok(Value::List(l.to_owned()))
                }
            } else {
                l.push(args[1].to_owned());
                Ok(Value::List(l.to_owned()))
            }
        } else if let Value::String(mut s) = args[0].clone() {
            s.push_str(&format!("{}", &args[1]));
            Ok(Value::String(s.to_owned()))
        } else if let Value::Object(mut map) = args[0].clone() {
            if let Value::String(s) = &args[1] {
                if args.len() != 3 {
                    if map.contains_key(s) {
                        Err(
                            crate::error!("Key `", s,"` is already present in the targetted object.")
                        )
                    } else {
                        map.insert(s.to_owned(), Value::Nil);
                        Ok(
                            Value::Object(
                                map
                            )
                        )
                    }
                } else {
                    map.insert(s.to_owned(), args[2].to_owned());
                    Ok(
                        Value::Object(
                            map
                        )
                    )
                }
            } else {
                Err(
                    crate::error!("Invalid argument, expected string, found", (&args[1].get_type()))
                )
            }
        } else {
            Err(
                crate::error!("Invalid argument, expected string, list or object, found", (&args[0].get_type()))
            )
        }
    }

    pub fn foreach(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 2 {
            return Err(
                crate::error!("Invalid number of arguments, expected 2|3, found", (args.len()))
            )
        }

        if let Value::Function(func_args, body) = &args[1] {
            if func_args.len() == 1 {
                if let Value::String(s) = &args[0] {
                    for c in s.chars() {
                        self.scopes.push(BTreeMap::new());
                        self.scopes.last_mut().unwrap().insert(func_args[0].to_owned(), (Value::String(format!("{}", c)), true));
                        self.eval_calls(&body.children)?;

                        self.scopes.pop();
                    }
                    Ok(Value::Nil)
                } else if let Value::List(l) = &args[0] {
                    for el in l {
                        self.scopes.push(BTreeMap::new());
                        self.scopes.last_mut().unwrap().insert(func_args[0].to_owned(), (el.to_owned(), true));
                        self.eval_calls(&body.children)?;

                        self.scopes.pop();
                    }
                    Ok(Value::Nil)
                } else {
                    Err(
                        error!("Invalid argument, expected string or list, found", (&args[0].get_type()))
                    )
                }
            } else if func_args.len() == 2 {
                if let Value::Object(map) = &args[0] {
                    for (key, value) in map{
                        self.scopes.push(BTreeMap::new());
                        let scp = self.scopes.last_mut().unwrap();
                        scp.insert(func_args[0].to_owned(), (Value::String(key.to_owned()), true));
                        scp.insert(func_args[1].to_owned(), (value.to_owned(), true));
                        self.eval_calls(&body.children)?;
                        self.scopes.pop();
                    }
                    Ok(Value::Nil)
                } else {
                    Err(
                        error!("Invalid argument, expected object, found", (&args[0].get_type()))
                    )
                }
            } else {
                Err(
                    error!("Invalid argument, expected function(x) or function(k, v), found", (&args[1].get_type()))
                )
            }
        } else {
            Err(
                error!("Invalid argument, expected function, found", (&args[1].get_type()))
            )
        }
    }

    pub fn len(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 1 {
            return Err(
                crate::error!("Invalid number of arguments, expected 1, found", (args.len()))
            )
        }

        if let Value::List(l) = &args[0] {
            Ok(Value::Int(l.len() as i32))
        } else if let Value::String(s) = &args[0] {
            Ok(
                Value::Int(s.len() as i32)
            )
        } else {
            Err(
                crate::error!("Invalid argument, expected string or list, found", (&args[0].get_type()))
            )
        }
    }
    pub fn pop(&mut self, args: &Vec<Value>) -> crate::Result<Value> {
        if args.len() != 1 {
            return Err(
                crate::error!("Invalid number of arguments, expected 1, found", (args.len()))
            )
        }

        if let Value::List(mut l) = args[0].clone() {
            l.pop();
            Ok(Value::List(
                l.to_owned()
            ))
        } else if let Value::String(mut s) = args[0].clone() {
            s.pop();
            Ok(Value::String(s.to_owned()))
        } else {
            Err(
                crate::error!("Invalid argument, expected string or list, found", (&args[0].get_type()))
            )
        }
    }
}