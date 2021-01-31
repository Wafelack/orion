use crate::interpreter::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::parser::node::{Node, NodeType};

impl Interpreter {
    pub fn eval_condition(&mut self, args: &Vec<Node>) -> crate::Result<Value> {

        if args.len() != 2 && args.len() != 3 {
            return Err(
                crate::error!("Invalid number of arguments, expected 2|3, found", (args.len()))
            )
        }

        let r_condition = self.to_value(&args[0])?;

        let condition = if let Value::Bool(b) = r_condition {
            b
        } else {
            false
        };

        if condition {
            if let NodeType::Scope = &args[1].ntype {
                self.eval_scope(&args[1])
            } else {
                Err(
                    crate::error!("Invalid argument, expected scope, found", (&args[1].ntype.stringy_type()))
                )
            }
        } else {
            if args.len() == 3 {
                if let NodeType::Scope = &args[2].ntype {
                    self.eval_scope(&args[2])
                } else {
                    Err(
                        crate::error!("Invalid argument, expected scope, found", (&args[2].ntype.stringy_type()))
                    )
                }
            } else {
                Ok(Value::Nil)
            }
        }

    }
}