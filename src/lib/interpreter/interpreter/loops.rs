use crate::interpreter::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::parser::node::{Node, NodeType};

impl Interpreter {

    pub fn eval_loop(&mut self, args: &Vec<Node>) -> crate::Result<Value> {
        if args.len() != 2 {
            return Err(
                crate::error!("Invalid number of arguments, expected 2, found", (args.len()))
            )
        }

        if let NodeType::Scope = &args[1].ntype {
        } else {
            return Err(
                crate::error!("Invalid argument, expected scope, found", (&args[1].ntype.stringy_type()))
            );
        }

        let mut cdn = self.conditionning(&args[0])?;

        while cdn {
            self.eval_scope(&args[1])?;
            cdn = self.conditionning(&args[0])?;
        }

        Ok(Value::Nil)
    }

    fn conditionning(&mut self, cdn: &Node) -> crate::Result<bool> {
        let r_condition = self.to_value(cdn)?;

        if let Value::Bool(b) = r_condition {
            Ok(b)
        } else {
            Ok(false)
        }
    }

}

