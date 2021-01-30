use crate::interpreter::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::parser::node::{Node, NodeType};

impl Interpreter {

    pub fn eval_lambda(&mut self, args: &Vec<Node>) -> crate::Result<Value> {

        if args.len() != 2 {
            return Err(
                error!("Invalid number of arguments, expected 2, found", (args.len()))
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
                        error!("Invalid argument, expected identifier, found", (child.ntype.stringy_type()))
                    )
                }
            }

            toret

        } else {
            return Err(
                error!("Invalid argument, expected identifier, found", (&args[0].ntype.stringy_type()))
            )
        };

        if let NodeType::Scope = &args[1].ntype {
        } else {
            return Err(
                error!("Invalid argument, expected scope, found", (&args[1].ntype.stringy_type()))
            )
        }

        Ok(Value::Function(s_args, args[1].clone()))
    }

}