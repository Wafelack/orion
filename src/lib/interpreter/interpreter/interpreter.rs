use std::collections::BTreeMap;
use crate::interpreter::value::Value;
use crate::parser::node::{Node, NodeType};

pub struct Interpreter {
    pub scopes: Vec<BTreeMap<String, (Value, bool)>>,
    pub ast: Node,
}

impl Interpreter {
    pub fn new(ast: Node) -> Self {
        Self {
            scopes: vec![BTreeMap::new()],
            ast,
        }
    }
    pub fn process_ast(&mut self, ast: &Node) -> crate::Result<()> {
        self.eval_calls(&ast.children)?;
        Ok(())
    }
    pub fn get_ast(&mut self) -> Node {
        self.ast.clone()
    }
    pub fn eval(&mut self) -> crate::Result<()> {
        let ast = &self.get_ast();
        self.eval_scope(ast)?;
        Ok(())
    }
    pub fn eval_scope(&mut self, scope: &Node) -> crate::Result<Value> {
        self.scopes.push(BTreeMap::new());
        let toret = self.eval_calls(&scope.children)?;
        self.scopes.pop();

        Ok(toret)
    }

    pub fn to_value(&mut self, node: &Node) -> crate::Result<Value> {

        Ok(match &node.ntype {
            NodeType::Bool(b) => Value::Bool(*b),
            NodeType::Scope => self.eval_scope(node)?,
            NodeType::Int(i) => Value::Int(*i),
            NodeType::Float(f) => Value::Float(*f),
            NodeType::Nil => Value::Nil,
            NodeType::String(s) => Value::String(s.to_owned()),
            NodeType::FunctionCall(f) => self.eval_call(&f, &node.children)?,
            NodeType::Identifier(id) => self.identifier(&id)?,
        })
    }

    pub fn identifier(&mut self, id: &str) -> crate::Result<Value> {

        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains_key(id) {
                return Ok(self.scopes[i][id].0.clone())
            }
        }

        Err(
            crate::error!("Cannot find", id, "in this scope.")
        )
    }

    pub fn eval_calls(&mut self, children: &Vec<Node>) -> crate::Result<Value> {

        for i in 0..children.len() {
            let child = &children[i];

            if let NodeType::FunctionCall(s) = &child.ntype {

               if i == children.len() - 1  {
                   return self.eval_call(s, &child.children);
               } else if s == "return" {
                    return self.eval_call(s, &child.children);
               } else {
                    self.eval_call(s, &child.children)?;
               }

            } else {
                return Err(
                    crate::error!("This should not be called, please open an issue.", "Error code: ERR_INVALID_FUNCTION_CALL")
                );
            }
        }

        Ok(Value::Nil) // Should not be hit but rust requires it ¯\_(ツ)_/¯
    }

    pub fn eval_call(&mut self, name: &str, args: &Vec<Node>) -> crate::Result<Value> {
        match name {

            // variables
            "define" => {
                self.eval_def(&args, false)?;
                Ok(Value::Nil)
            },
            "var" =>{
                self.eval_def(&args, true)?;
                Ok(Value::Nil)
            }
            "set" => {
                self.eval_set(&args)?;
                Ok(Value::Nil)
            }

            // return

            "return" => if args.len() < 1 {
                Ok(Value::Nil)
            } else {
                Ok(self.to_value(&args[0])?)
            }

            // conditions

            "if" => self.eval_condition(&args),

            // loops 

            "while" => self.eval_loop(&args),

            "lambda" => self.eval_lambda(&args),

            // arithmetic

            "+" => self.eval_plus(&args),           
            "*" => self.eval_times(&args),      
            "/" => self.eval_div(&args),     
            "%" => self.eval_modulo(&args),     
            "-" => self.eval_minus(&args),     

            // boolean algebra

            "=" => self.eval_eq(&args),
            "!=" => self.eval_neq(&args),
            "<" => self.eval_le(&args),
            ">" => self.eval_ge(&args),
            "<=" => self.eval_leq(&args),
            ">=" => self.eval_geq(&args),
            "|" => self.eval_or(&args),
            "&" => self.eval_and(&args),
            "!" => self.eval_not(&args),

            // std
            _ => {
                let mut valued = vec![];

                for arg in args {
                    valued.push(self.to_value(arg)?);
                }

                match name {
                    // io
                    "print" => self.print(&valued),
                    "puts" => self.puts(&valued),
                    "eprint" => self.eprint(&valued),
                    "eputs" => self.eputs(&valued),
                    "input" => self.input(&valued),

                    // misc
                    "assert" => self.assert(&valued),
                    "typeof" => self._typeof(&valued),
                    "import" => self.import(&valued),

                    // Collections
                    "list" => self.list(&valued),
                    "push" => self.push(&valued),
                    "object" => self.object(&valued),
                    "pop" => self.pop(&valued),
                    "length" => self.len(&valued),
                    "@" => self.index(&valued),

                    // fs
                    "fs:exists?" => self.exists(&valued),

                    _ => self.scope_function(name, &valued),
                }
            }
        }
    }
    
    
}