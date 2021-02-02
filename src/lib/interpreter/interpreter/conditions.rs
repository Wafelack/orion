use crate::interpreter::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::parser::node::{Node, NodeType};

impl Interpreter {
    pub fn eval_match(&mut self, args: &Vec<Node>) -> crate::Result<Value> {
        if args.len() != 2 {
            return Err(
                crate::error!("Invalid number of arguments, expected 2, found", (args.len()))
            )
        }

        let value = self.to_value(&args[0])?;

        if let NodeType::Scope = &args[1].ntype {
            let cases = &args[1].children;

            for case in cases {

                if let NodeType::FunctionCall(s) = &case.ntype {
                    if s == "=>" {
                        if case.children.len() != 2 {
                            return Err(
                                crate::error!("Invalid number of arguments, expected 2, found", (&case.children.len()))
                            )
                        }

                        let pattern = self.to_value(&case.children[0])?;

                        if let NodeType::Scope = &case.children[1].ntype {
                            if pattern == value {
                                return self.eval_scope(&case.children[1]);
                            } else {
                                continue;
                            }
                        } else {
                            return Err(
                                crate::error!("Invalid argument, expected scope, found ", (&case.children[1].ntype.stringy_type()))
                            )
                        }

                    } else if s == "_"  {
                        if case.children.len() != 1 {
                            return Err(
                                crate::error!("Invalid number of arguments, expected 1, found", (case.children.len()))
                            )
                        }
                        if let NodeType::Scope = &case.children[0].ntype {
                            return self.eval_scope(&case.children[0]);
                        } else {
                            return Err(
                                crate::error!("Invalid argument, expected scope, found ", (&case.children[0].ntype.stringy_type()))
                            )
                        }

                    } else {
                        return Err(
                            crate::error!("Invalid function call, expected `=>` or `_`, found ", (&case.ntype.stringy_type()))
                        )
                    }
                } else {
                    return Err(
                        crate::error!("Invalid argument, expected function call, found ", (&case.ntype.stringy_type()))
                    )
                }
            }

        } else {
            return Err(
                crate::error!("Invalid argument, expected scope, found ", (&args[1].ntype.stringy_type()))
            )
        }

        Ok(Value::Nil)

    }
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