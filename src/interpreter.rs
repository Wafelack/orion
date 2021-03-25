use crate::{error, OrionError, Result, parser::{Node, NType}};
use std::collections::HashMap;

pub enum Value {
    Integer(i32),
    Single(f32),
    Boolean(bool),
    Str(String),
    Maybe(Option<Value>),
    Function(Node, Vec<String>, Vec<HashMap<String, Value>>)
}

impl Value {
    pub fn get_type()
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
