use crate::{error, OrionError, Result, parser::{Node, NType}};
use std::collections::HashMap;

pub enum Value {
    Integer(i32),
    Single(f32),
    Boolean(bool),
    Str(String),
    Maybe(Box<Option<Value>>),
    Function(Node /* Body */,
            Vec<String> /* Args */,
            Vec<HashMap<String, Value>> /* Scopes */,

            String /* Return type */,
            Vec<String>, /* Args type */)
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
            Self::Function(_, _, _, ret, args) => {
                let mut to_ret = "".to_owned();

                args.iter().for_each(|a| to_ret.push_str(&format!("{} -> ", a)));

                to_ret.push_str(&ret);
                to_ret
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

        error!("This is a bug please report it with the following information: UNTRIGGERED_RETURN: [{}:{}]", file!(), line!())
    }
    fn eval_call(&mut self, call: &Node) -> Result<Value> {

        if let NType::Ident(function) = &call.ntype {
           
            match function.as_str() {
                _ => unimplemented!(),
            }

        } else {
                error!("This is a bug, please report it with the following information: INVALID_IDENTIFIER: [{}:{}]", file!(), line!())
        }

    }
}
