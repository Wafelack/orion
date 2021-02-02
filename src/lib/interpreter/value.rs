use std::collections::BTreeMap;
use std::{fmt, fmt::{Display, Formatter}};
use crate::parser::node::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Object(BTreeMap<String, Value>),
    List(Vec<Value>),
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
    Function(Vec<String>, Node),
    Nil
}

impl Value {
    pub fn get_type(&self) -> String {
        match self {
            Self::Object(_) => "object",
            Self::List(_) => "list",
            Self::Int(_) => "int",
            Self::Float(_) => "float",
            Self::String(_) => "string",
            Self::Bool(_) => "bool",
            Self::Function(_,_) => "function",
            Self::Nil => "nil"
        }.to_owned()
    }
    pub fn cast_to(&self, typ: &str) -> crate::Result<Value> {
        match typ {
            "string" => {
                match self {
                    Self::List(v) => {
                        let mut toret = String::new();

                        for val in v {
                            toret.push_str(&format!("{}", val));
                        }
                        Ok(
                            Value::String(toret)
                        )
                    }
                    Self::Float(f) => Ok(Value::String(format!("{}", f))),
                    Self::Int(i) => Ok(Value::String(format!("{}", i))),
                    Self::Bool(b) => Ok(Value::String(format!("{}", b))),
                    _ => Ok(Value::String(format!(""))),
                }
            }
            "float" => {
                match self {
                    Self::Float(f) => Ok(Value::Float(*f)),
                    Self::Int(i) => Ok(Value::Float(*i as f32)),
                    Self::Bool(b) => Ok(Value::Float(if !b { 0.} else {1.})),
                    Self::String(s) => Ok(Value::Float(s.parse::<f32>().unwrap_or(0.))),
                    _ => Ok(Value::Float(0.)),
                }
            }
            "int" => {
                match self {
                    Self::Float(f) => Ok(Value::Int(f.floor() as i32)),
                    Self::Int(i) => Ok(Value::Int(*i)),
                    Self::Bool(b) => Ok(Value::Int(*b as i32)),
                    Self::String(s) => Ok(Value::Int(s.parse::<i32>().unwrap_or(0))),
                    _ => Ok(Value::Int(0)),
                }
            }
            "list" => {
                Ok(
                    Value::List(vec![self.to_owned()])
                )
            }
            "bool" => {
                match self {
                    Self::Float(f) => Ok(Value::Bool(if *f == 0. { false } else {true})),
                    Self::Int(i) => Ok(Value::Bool(if *i == 0 { false } else {true})),
                    Self::Bool(b) => Ok(Value::Bool(*b)),
                    Self::String(s) => Ok(Value::Bool(s.parse::<bool>().unwrap_or(false))),
                    _ => Ok(Value::Bool(false)),
                } }
            _ => {
                Ok(Value::Nil)
            }
        }
    }
}


impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::Object(map) => write!(f, "{}", jsonize(map, 0)),
            Value::List(arr) => {
                write!(f, "[")?;
                for i in 0..arr.len() {
                    if i == arr.len() - 1{
                        write!(f, "{}]", arr[i])?;
                    } else {
                        write!(f, "{}, ", arr[i])?;
                    }
                }   
                Ok(())
            }
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Function(args, _) => write!(f, "function({})", args.join(", ")),
            Value::Nil => write!(f, "nil"),
        }
    }
}

fn jsonize(map: &BTreeMap<String, Value>, level: usize) -> String {
    let mut toret = String::from("{\n");

    for (key, value) in map {
        toret.push_str(&format!("{}{} => ", get_indents(level + 1), key));

        match value {
            Value::String(s) => toret.push_str(&format!("{},\n",s)),
            Value::Int(i) => toret.push_str(&format!("{},\n", i)),
            Value::Float(f) => toret.push_str(&format!("{},\n", f)),
            Value::Bool(b) => toret.push_str(&format!("{},\n", b)),
            Value::List(arr) => {
                toret.push_str("[");
                for i in 0..arr.len() {
                    if i == arr.len() - 1{
                        toret.push_str(&format!("{}],\n",arr[i]));
                    } else {
                        toret.push_str(&format!("{}, ",arr[i]));
                    }
                }
            }
            Value::Object(map) => toret.push_str(&format!("{}", jsonize(map, level + 1))),
            Value::Nil => toret.push_str("nil,\n"),
            Value::Function(args, _) => toret.push_str(&format!("function({})\n", args.join(", "))),
        }
    }
    toret.push_str(&format!("{}}},\n", get_indents(level)));
    toret
}

fn get_indents(level: usize) -> String {
    let mut toret = String::new();

    for _ in 0..level {
        toret.push_str("\t");
    }
    toret
}