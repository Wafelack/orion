/* use crate::{error, OrionError, Result, parser::{Node, NType}};
use std::collections::HashMap;

macro_rules! check_args {
    ($node:tt, $name:literal, $($len:tt),*) => {
        {
            let mut lengths = vec![];

            $(
                lengths.push($len);
             )*

            let mut formatted = String::new();

            lengths.iter().enumerate().for_each(|(idx, val)| if idx == 0 {
                formatted.push_str(&format!("{}", val));
            } else if idx == lengths.len() - 1 {
                formatted.push_str(&format!(" or {}", val));
            } else {
                formatted.push_str(&format!(", {}", val));
            });

            if lengths.contains(&$node.children.len()) {
                Ok(())
            } else {
                error!("Function `{}` takes {} arguments but {} arguments were supplied.", $name, formatted, $node.children.len())
            }
        }
    }
}


macro_rules! check_type {
    ($node:tt, $($types:literal),*) => {
        {
            let mut types = vec![];

            $(
                types.push($types);
             )*

            let mut formatted = String::new();

            types.iter().enumerate().for_each(|(idx, val)| if idx == 0 {
                formatted.push_str(&format!("{}", val));
            } else if idx == types.len() - 1 {
                formatted.push_str(&format!(" or {}", val));
            } else {
                formatted.push_str(&format!(", {}", val));
            });

            if types.contains(&$node.type_lit().as_str()) {
                Ok(())
            } else {
                error!("Expect an expression of type `{}` but found one of type {}.", formatted, $node.type_lit())
            }
        }
    }

}

pub enum Value {
    Integer(i32),
    Single(f32),
    Boolean(bool),
    Str(String),
    Maybe(Box<Option<Value>>),
    Lambda(Node /* Body */,
            Vec<String> /* Args */,
            HashMap<String, Value> /* Scopes */)
}

impl Value {
    pub fn get_type(&self) -> String {
        match self {
            Self::Integer(_) => "Integer".to_string(),
            Self::Single(_) => "Single".to_string(),
            Self::Boolean(_) => "Boolean".to_string(),
            Self::Str(_) => "String".to_string(),
            Self::Maybe(b) => match &**b {
                Some(v) => format!("Maybe {}", v.get_type()),
                None => "Maybe".to_string(),
            }
            Self::Lambda(_,_,_) => {
                format!("Lambda")
            }
        }
    }
}

pub struct Interpreter {
    pub input: Node,
    pub scopes: Vec<HashMap<String, Value>>,
}

impl Interpreter {
    pub fn new(ast: Node) -> Self {
        Self {
            scopes: vec![HashMap::new()],
            input: ast,
        }
    }
    fn process_ast(&mut self, ast: &Node) -> Result<Value> {
        self.eval_calls(&ast.children)
    }
    pub fn eval(&mut self) -> Result<Value> {
        let ast = self.input.clone();
        self.eval_calls(&ast.children)
    }
    fn eval_calls(&mut self, calls: &Vec<Node>) -> Result<Value> {
        for (idx, call) in calls.iter().enumerate() {
            if let NType::Ident(i) = &call.ntype {
                if idx == calls.len() - 1 {
                    return self.eval_call(&call);
                } else if i == "return" {
                    return self.eval_call(&call);
                } else {
                    self.eval_call(&call)?;
                }
            } else {
                return error!("This is a bug, please report it with the following information: INVALID_IDENTIFIER: [{}:{}]", file!(), line!());
            }
        }

        bug!("UNTRIGGERED_RETURN")
    }
    fn eval_call(&mut self, call: &Node) -> Result<Value> {

        if let NType::Ident(function) = &call.ntype {

            match function.as_str() {
                "def!" => self.def(&call),
                _ => unimplemented!(),
            }

        } else {
            bug!("INVALID_IDENTIFIER")
        }

    }
    fn def(&mut self, node: &Node) -> Result<Value> {

        check_args!(node, "def!", 2)?;
        let raw_name = &node.children[0];

        check_type!(raw_name, "Identifier")?;

        if let NType::Ident(identifier) = &raw_name.ntype {


            Ok(Value::Integer(0))
        } else {
            bug!("INVALID_IDENTIFIER")
        }

    }
} */
