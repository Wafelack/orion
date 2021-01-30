use crate::interpreter::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::parser::node::{Node, NodeType};

impl Interpreter {
    pub fn eval_eq(&mut self, args: &Vec<Node>) -> crate::Result<Value> {

        if args.len() != 2 {
            return Err(
                error!("Invalid number of arguments, expected 2, found", (args.len()))
            );
        }

        let lhs = match &args[0].ntype {
            NodeType::Int(i) => Value::Int(*i),
            NodeType::Float(f) => Value::Float(*f),
            NodeType::Bool(b) => Value::Bool(*b),
            NodeType::String(s) => Value::String(s.to_owned()),
            NodeType::Nil => Value::Nil,
            NodeType::Scope => self.eval_scope(&args[0])?,
            NodeType::FunctionCall(s) => self.eval_call(&s, &args[0].children)?,
            NodeType::Identifier(s) => self.identifier(&s)?,
        };
        let rhs = match &args[1].ntype {
            NodeType::Int(i) => Value::Int(*i),
            NodeType::Float(f) => Value::Float(*f),
            NodeType::Bool(b) => Value::Bool(*b),
            NodeType::String(s) => Value::String(s.to_owned()),
            NodeType::Nil => Value::Nil,
            NodeType::Scope => self.eval_scope(&args[0])?,
            NodeType::FunctionCall(s) => self.eval_call(&s, &args[0].children)?,
            NodeType::Identifier(s) => self.identifier(&s)?,
        };



        if std::mem::discriminant(&lhs) != std::mem::discriminant(&rhs) {
            return Ok(Value::Bool(false));
        }

        Ok(match lhs {
            Value::Int(lh) => match rhs {
                Value::Int(rh) => Value::Bool(lh == rh),
                _ => Value::Bool(false),
            }
            Value::Float(lh) => match rhs {
                Value::Float(rh) => Value::Bool(lh == rh),
                _ => Value::Bool(false),
            }
            Value::String(lh) => match rhs {
                Value::String(rh) => Value::Bool(lh == rh),
                _ => Value::Bool(false),
            }
            Value::Nil => match rhs {
                Value::Nil => Value::Nil,
                _ => Value::Bool(false),
            }
            Value::List(lh) => match rhs {
                Value::List(rh) => Value::Bool(lh == rh),
                _ => Value::Bool(false),
            }
            Value::Object(lh) => match rhs {
                Value::Object(rh) => Value::Bool(rh == lh),
                _ => Value::Bool(false),
            }
            _ => Value::Bool(false)
        })
    }

    pub fn eval_neq(&mut self, args: &Vec<Node>) -> crate::Result<Value> {

        if args.len() != 2 {
            return Err(
                error!("Invalid number of arguments, expected 2, found", (args.len()))
            );
        }

        let lhs = match &args[0].ntype {
            NodeType::Int(i) => Value::Int(*i),
            NodeType::Float(f) => Value::Float(*f),
            NodeType::Bool(b) => Value::Bool(*b),
            NodeType::String(s) => Value::String(s.to_owned()),
            NodeType::Nil => Value::Nil,
            NodeType::Scope => self.eval_scope(&args[0])?,
            NodeType::FunctionCall(s) => self.eval_call(&s, &args[0].children)?,
            NodeType::Identifier(s) => self.identifier(&s)?,
        };
        let rhs = match &args[1].ntype {
            NodeType::Int(i) => Value::Int(*i),
            NodeType::Float(f) => Value::Float(*f),
            NodeType::Bool(b) => Value::Bool(*b),
            NodeType::String(s) => Value::String(s.to_owned()),
            NodeType::Nil => Value::Nil,
            NodeType::Scope => self.eval_scope(&args[0])?,
            NodeType::FunctionCall(s) => self.eval_call(&s, &args[0].children)?,
            NodeType::Identifier(s) => self.identifier(&s)?,
        };



        if std::mem::discriminant(&lhs) != std::mem::discriminant(&rhs) {
            return Ok(Value::Nil);
        }

        Ok(match lhs {
            Value::Int(lh) => match rhs {
                Value::Int(rh) => Value::Bool(lh != rh),
                _ => Value::Nil,
            }
            Value::Float(lh) => match rhs {
                Value::Float(rh) => Value::Bool(lh != rh),
                _ => Value::Nil,
            }
            Value::String(lh) => match rhs {
                Value::String(rh) => Value::Bool(lh != rh),
                _ => Value::Nil,
            }
            Value::Nil => match rhs {
                Value::Nil => Value::Bool(false),
                _ => Value::Nil,
            }
            Value::List(lh) => match rhs {
                Value::List(rh) => Value::Bool(lh != rh),
                _ => Value::Nil,
            }
            Value::Object(lh) => match rhs {
                Value::Object(rh) => Value::Bool(rh != lh),
                _ => Value::Nil,
            }
            _ => Value::Nil
        })
    }

    pub fn eval_le(&mut self, args: &Vec<Node>) -> crate::Result<Value> {

        if args.len() != 2 {
            return Err(
                error!("Invalid number of arguments, expected 2, found", (args.len()))
            );
        }

        let lhs = match &args[0].ntype {
            NodeType::Int(i) => Value::Int(*i),
            NodeType::Float(f) => Value::Float(*f),
            NodeType::Bool(b) => Value::Bool(*b),
            NodeType::String(s) => Value::String(s.to_owned()),
            NodeType::Nil => Value::Nil,
            NodeType::Scope => self.eval_scope(&args[0])?,
            NodeType::FunctionCall(s) => self.eval_call(&s, &args[0].children)?,
            NodeType::Identifier(s) => self.identifier(&s)?,
        };
        let rhs = match &args[1].ntype {
            NodeType::Int(i) => Value::Int(*i),
            NodeType::Float(f) => Value::Float(*f),
            NodeType::Bool(b) => Value::Bool(*b),
            NodeType::String(s) => Value::String(s.to_owned()),
            NodeType::Nil => Value::Nil,
            NodeType::Scope => self.eval_scope(&args[0])?,
            NodeType::FunctionCall(s) => self.eval_call(&s, &args[0].children)?,
            NodeType::Identifier(s) => self.identifier(&s)?,
        };



        if std::mem::discriminant(&lhs) != std::mem::discriminant(&rhs) {
            return Ok(Value::Nil);
        }

        Ok(match lhs {
            Value::Int(lh) => match rhs {
                Value::Int(rh) => Value::Bool(lh < rh),
                _ => Value::Nil,
            }
            Value::Float(lh) => match rhs {
                Value::Float(rh) => Value::Bool(lh < rh),
                _ => Value::Nil,
            }
            _ => Value::Nil
        })
    }

    pub fn eval_leq(&mut self, args: &Vec<Node>) -> crate::Result<Value> {

        if args.len() != 2 {
            return Err(
                error!("Invalid number of arguments, expected 2, found", (args.len()))
            );
        }

        let lhs = match &args[0].ntype {
            NodeType::Int(i) => Value::Int(*i),
            NodeType::Float(f) => Value::Float(*f),
            NodeType::Bool(b) => Value::Bool(*b),
            NodeType::String(s) => Value::String(s.to_owned()),
            NodeType::Nil => Value::Nil,
            NodeType::Scope => self.eval_scope(&args[0])?,
            NodeType::FunctionCall(s) => self.eval_call(&s, &args[0].children)?,
            NodeType::Identifier(s) => self.identifier(&s)?,
        };
        let rhs = match &args[1].ntype {
            NodeType::Int(i) => Value::Int(*i),
            NodeType::Float(f) => Value::Float(*f),
            NodeType::Bool(b) => Value::Bool(*b),
            NodeType::String(s) => Value::String(s.to_owned()),
            NodeType::Nil => Value::Nil,
            NodeType::Scope => self.eval_scope(&args[0])?,
            NodeType::FunctionCall(s) => self.eval_call(&s, &args[0].children)?,
            NodeType::Identifier(s) => self.identifier(&s)?,
        };



        if std::mem::discriminant(&lhs) != std::mem::discriminant(&rhs) {
            return Ok(Value::Nil);
        }

        Ok(match lhs {
            Value::Int(lh) => match rhs {
                Value::Int(rh) => Value::Bool(lh <= rh),
                _ => Value::Nil,
            }
            Value::Float(lh) => match rhs {
                Value::Float(rh) => Value::Bool(lh <= rh),
                _ => Value::Nil,
            }
            _ => Value::Nil
        })
    }

    pub fn eval_ge(&mut self, args: &Vec<Node>) -> crate::Result<Value> {

        if args.len() != 2 {
            return Err(
                error!("Invalid number of arguments, expected 2, found", (args.len()))
            );
        }

        let lhs = match &args[0].ntype {
            NodeType::Int(i) => Value::Int(*i),
            NodeType::Float(f) => Value::Float(*f),
            NodeType::Bool(b) => Value::Bool(*b),
            NodeType::String(s) => Value::String(s.to_owned()),
            NodeType::Nil => Value::Nil,
            NodeType::Scope => self.eval_scope(&args[0])?,
            NodeType::FunctionCall(s) => self.eval_call(&s, &args[0].children)?,
            NodeType::Identifier(s) => self.identifier(&s)?,
        };
        let rhs = match &args[1].ntype {
            NodeType::Int(i) => Value::Int(*i),
            NodeType::Float(f) => Value::Float(*f),
            NodeType::Bool(b) => Value::Bool(*b),
            NodeType::String(s) => Value::String(s.to_owned()),
            NodeType::Nil => Value::Nil,
            NodeType::Scope => self.eval_scope(&args[0])?,
            NodeType::FunctionCall(s) => self.eval_call(&s, &args[0].children)?,
            NodeType::Identifier(s) => self.identifier(&s)?,
        };



        if std::mem::discriminant(&lhs) != std::mem::discriminant(&rhs) {
            return Ok(Value::Nil);
        }

        Ok(match lhs {
            Value::Int(lh) => match rhs {
                Value::Int(rh) => Value::Bool(lh > rh),
                _ => Value::Nil,
            }
            Value::Float(lh) => match rhs {
                Value::Float(rh) => Value::Bool(lh > rh),
                _ => Value::Nil,
            }
            _ => Value::Nil
        })
    }

    pub fn eval_geq(&mut self, args: &Vec<Node>) -> crate::Result<Value> {

        if args.len() != 2 {
            return Err(
                error!("Invalid number of arguments, expected 2, found", (args.len()))
            );
        }

        let lhs = match &args[0].ntype {
            NodeType::Int(i) => Value::Int(*i),
            NodeType::Float(f) => Value::Float(*f),
            NodeType::Bool(b) => Value::Bool(*b),
            NodeType::String(s) => Value::String(s.to_owned()),
            NodeType::Nil => Value::Nil,
            NodeType::Scope => self.eval_scope(&args[0])?,
            NodeType::FunctionCall(s) => self.eval_call(&s, &args[0].children)?,
            NodeType::Identifier(s) => self.identifier(&s)?,
        };
        let rhs = match &args[1].ntype {
            NodeType::Int(i) => Value::Int(*i),
            NodeType::Float(f) => Value::Float(*f),
            NodeType::Bool(b) => Value::Bool(*b),
            NodeType::String(s) => Value::String(s.to_owned()),
            NodeType::Nil => Value::Nil,
            NodeType::Scope => self.eval_scope(&args[0])?,
            NodeType::FunctionCall(s) => self.eval_call(&s, &args[0].children)?,
            NodeType::Identifier(s) => self.identifier(&s)?,
        };



        if std::mem::discriminant(&lhs) != std::mem::discriminant(&rhs) {
            return Ok(Value::Nil);
        }

        Ok(match lhs {
            Value::Int(lh) => match rhs {
                Value::Int(rh) => Value::Bool(lh >= rh),
                _ => Value::Nil,
            }
            Value::Float(lh) => match rhs {
                Value::Float(rh) => Value::Bool(lh >= rh),
                _ => Value::Nil,
            }
            _ => Value::Nil
        })
    }

    pub fn eval_or(&mut self, args: &Vec<Node>) -> crate::Result<Value> {

        if args.len() != 2 {
            return Err(
                error!("Invalid number of arguments, expected 2, found", (args.len()))
            );
        }

        let lhs = match &args[0].ntype {
            NodeType::Int(i) => Value::Int(*i),
            NodeType::Float(f) => Value::Float(*f),
            NodeType::Bool(b) => Value::Bool(*b),
            NodeType::String(s) => Value::String(s.to_owned()),
            NodeType::Nil => Value::Nil,
            NodeType::Scope => self.eval_scope(&args[0])?,
            NodeType::FunctionCall(s) => self.eval_call(&s, &args[0].children)?,
            NodeType::Identifier(s) => self.identifier(&s)?,
        };
        let rhs = match &args[1].ntype {
            NodeType::Int(i) => Value::Int(*i),
            NodeType::Float(f) => Value::Float(*f),
            NodeType::Bool(b) => Value::Bool(*b),
            NodeType::String(s) => Value::String(s.to_owned()),
            NodeType::Nil => Value::Nil,
            NodeType::Scope => self.eval_scope(&args[0])?,
            NodeType::FunctionCall(s) => self.eval_call(&s, &args[0].children)?,
            NodeType::Identifier(s) => self.identifier(&s)?,
        };



        if std::mem::discriminant(&lhs) != std::mem::discriminant(&rhs) {
            return Ok(Value::Bool(false));
        }

        Ok(match lhs {
            Value::Bool(lh) => match rhs {
                Value::Bool(rh) => Value::Bool(lh || rh),
                _ => Value::Bool(false),
            }
            _ => Value::Bool(false)
        })
    }

    pub fn eval_and(&mut self, args: &Vec<Node>) -> crate::Result<Value> {

        if args.len() != 2 {
            return Err(
                error!("Invalid number of arguments, expected 2, found", (args.len()))
            );
        }

        let lhs = match &args[0].ntype {
            NodeType::Int(i) => Value::Int(*i),
            NodeType::Float(f) => Value::Float(*f),
            NodeType::Bool(b) => Value::Bool(*b),
            NodeType::String(s) => Value::String(s.to_owned()),
            NodeType::Nil => Value::Nil,
            NodeType::Scope => self.eval_scope(&args[0])?,
            NodeType::FunctionCall(s) => self.eval_call(&s, &args[0].children)?,
            NodeType::Identifier(s) => self.identifier(&s)?,
        };
        let rhs = match &args[1].ntype {
            NodeType::Int(i) => Value::Int(*i),
            NodeType::Float(f) => Value::Float(*f),
            NodeType::Bool(b) => Value::Bool(*b),
            NodeType::String(s) => Value::String(s.to_owned()),
            NodeType::Nil => Value::Nil,
            NodeType::Scope => self.eval_scope(&args[0])?,
            NodeType::FunctionCall(s) => self.eval_call(&s, &args[0].children)?,
            NodeType::Identifier(s) => self.identifier(&s)?,
        };



        if std::mem::discriminant(&lhs) != std::mem::discriminant(&rhs) {
            return Ok(Value::Bool(false));
        }

        Ok(match lhs {
            Value::Bool(lh) => match rhs {
                Value::Bool(rh) => Value::Bool(lh && rh),
                _ => Value::Bool(false),
            }
            _ => Value::Bool(false)
        })
    }
}