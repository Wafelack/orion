use crate::interpreter::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::parser::node::{Node, NodeType};

impl Interpreter {
    pub fn eval_plus(&mut self, args: &Vec<Node>) -> crate::Result<Value> {

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
        



        Ok(match lhs {
            Value::Int(lh) => match rhs {
                Value::Int(rh) => Value::Int(lh + rh),
                Value::Float(rh) => Value::Float(lh as f32 + rh),
                _ => Value::Nil,
            }
            Value::Float(lh) => match rhs {
                Value::Int(rh) => Value::Float(lh + rh as f32),
                Value::Float(rh) => Value::Float(lh + rh),
                _ => Value::Nil,
            }
            Value::String(lh) => match rhs {
                Value::String(rh) => Value::String(format!("{}{}", lh, rh)),
                _ => Value::Nil,
            }
            _ => Value::Nil
        })
    }

    pub fn eval_minus(&mut self, args: &Vec<Node>) -> crate::Result<Value> {


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

        Ok(match lhs {
            Value::Int(lh) => match rhs {
                Value::Int(rh) => Value::Int(lh - rh),
                Value::Float(rh) => Value::Float(lh as f32 - rh),
                _ => Value::Nil,
            }
            Value::Float(lh) => match rhs {
                Value::Int(rh) => Value::Float(lh - rh as f32),
                Value::Float(rh) => Value::Float(lh - rh),
                _ => Value::Nil,
            }
            _ => Value::Nil
        })
    }


    pub fn eval_times(&mut self, args: &Vec<Node>) -> crate::Result<Value> {

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



        Ok(match lhs {
            Value::Int(lh) => match rhs {
                Value::Int(rh) => Value::Int(lh * rh),
                Value::Float(rh) => Value::Float(lh as f32 * rh),
                _ => Value::Nil,
            }
            Value::Float(lh) => match rhs {
                Value::Int(rh) => Value::Float(lh * rh as f32),
                Value::Float(rh) => Value::Float(lh * rh),
                _ => Value::Nil,
            }
            _ => Value::Nil
        })
    }

    pub fn eval_div(&mut self, args: &Vec<Node>) -> crate::Result<Value> {

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



        Ok(match lhs {
            Value::Int(lh) => match rhs {
                Value::Int(rh) => Value::Int(lh / rh),
                Value::Float(rh) => Value::Float(lh as f32 / rh),
                _ => Value::Nil,
            }
            Value::Float(lh) => match rhs {
                Value::Int(rh) => Value::Float(lh / rh as f32),
                Value::Float(rh) => Value::Float(lh / rh),
                _ => Value::Nil,
            }
            _ => Value::Nil
        })
    }

    pub fn eval_modulo(&mut self, args: &Vec<Node>) -> crate::Result<Value> {

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



        Ok(match lhs {
            Value::Int(lh) => match rhs {
                Value::Int(rh) => Value::Int(lh % rh),
                Value::Float(rh) => Value::Float(lh as f32 % rh),
                _ => Value::Nil,
            }
            Value::Float(lh) => match rhs {
                Value::Int(rh) => Value::Float(lh % rh as f32),
                Value::Float(rh) => Value::Float(lh % rh),
                _ => Value::Nil,
            }
            _ => Value::Nil
        })
    }
}