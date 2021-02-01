use std::collections::BTreeMap;
use std::{fmt, fmt::{Display, Formatter}};
use crate::parser::node::Node;

#[derive(Clone, PartialEq)]
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