use crate::interpreter::interpreter::interpreter::Interpreter;
use crate::interpreter::value::Value;
use crate::parser::node::{Node, NodeType};

impl Interpreter {
    pub fn eval_set(&mut self, children: &Vec<Node>) -> crate::Result<()> {
        if children.len() != 2 {
            return Err(
                error!("Function `define` takes 2 arguments, but", (children.len()), "arguments were supplied.")
            )
        }


        if let NodeType::Identifier(name) = &children[0].ntype {
            
            if self.scopes.len() >= 1 {

                for i in (0..self.scopes.len()).rev() {
                    if self.scopes[i].contains_key(name) {
                        if !self.scopes[i][name].1 {
                            return Err(
                                error!("Attempted to assign value to a constant variable:", name)
                            );
                        } else {
                            let to_add = self.to_value(&children[1])?;
                            let val = self.scopes.get_mut(i).unwrap().get_mut(name).unwrap();
                            *val = (to_add, true);
                            return Ok(());
                        }
                    }
                }
                return Err(
                    error!("Attempted to assign value to undefined variable:", name)
                );
                
            } else {
                Err (
                    error!("No scopes available.")
                )
            }

        } else {
            return Err(
                error!("Invalid arguments. Expected identifier, found",(&children[0].ntype.stringy_type())) 
            );
        }
    }
    pub fn eval_def(&mut self, children: &Vec<Node>, mutable: bool) -> crate::Result<()> {

        if children.len() != 2 {
            return Err(
                error!("Function `define` takes 2 arguments, but", (children.len()), "arguments were supplied.")
            )
        }


        if let NodeType::Identifier(name) = &children[0].ntype {
            
            if self.scopes.len() >= 1 {
                if self.scopes.last().unwrap().contains_key(name) {
                    Err(
                        error!("Attempted to define an existing variable:", name)
                    )
                } else {
                    let to_add = self.to_value(&children[1])?;

                    self.scopes.last_mut().unwrap().insert(name.to_owned(), (to_add, mutable));

                    Ok(())
                }
            } else {
                Err (
                    error!("No scopes available.")
                )
            }

        } else {
            return Err(
                error!("Invalid arguments. Expected identifier, found",(&children[0].ntype.stringy_type())) 
            );
        }
    }
}