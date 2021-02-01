use crate::interpreter::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::parser::node::{Node, NodeType};
use std::collections::BTreeMap;

impl Interpreter {

    pub fn eval_lambda(&mut self, args: &Vec<Node>) -> crate::Result<Value> {

        if args.len() != 2 {
            return Err(
                crate::error!("Invalid number of arguments, expected 2, found", (args.len()))
            )
        }

        let s_args: Vec<String> = if let NodeType::FunctionCall(s) = &args[0].ntype {
            let mut toret = vec![s.to_owned()];

            for child in &args[0].children {
                if let NodeType::Identifier(s) = &child.ntype {
                    toret.push(
                        s.to_owned()
                    )
                } else {
                    return Err(
                        crate::error!("Invalid argument, expected identifier, found", (child.ntype.stringy_type()))
                    )
                }
            }

            toret

        } else {
            return Err(
                crate::error!("Invalid argument, expected identifier, found", (&args[0].ntype.stringy_type()))
            )
        };

        if let NodeType::Scope = &args[1].ntype {
        } else {
            return Err(
                crate::error!("Invalid argument, expected scope, found", (&args[1].ntype.stringy_type()))
            )
        }

        Ok(Value::Function(s_args, args[1].clone()))
    }

    pub fn scope_function(&mut self,name: &str, valued: &Vec<Value>) -> crate::Result<Value> {

        if let Value::Function(args, body) = self.identifier(name)? {
            if valued.len() != args.len() {
                return Err(
                    crate::error!("Invalid number of arguments, expected", (args.len()), ", found", (valued.len()))
                )
            }

            self.scopes.push(BTreeMap::new());

            for i in 0..valued.len() {
                self.scopes.last_mut().unwrap().insert(args[i].to_owned(), (valued[i].clone(), false));
            }

            let toret = self.eval_calls(&body.children);

            self.scopes.pop();
            toret

        } else {
            return Err(
                crate::error!("Invalid function call.")
            )
        }
    }
}