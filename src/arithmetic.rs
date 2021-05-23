use crate::{vm::{VM, Value}, OrionError, error, Result};

impl<const STACK_SIZE: usize> VM<STACK_SIZE> {
    pub fn add(&mut self) -> Result<Value> {
        let rhs = self.pop()?;
        let lhs = self.pop()?;

        match lhs {
            Value::Integer(lhs) => match rhs {
                Value::Integer(rhs) => Ok(Value::Integer(lhs + rhs)),
                _ => error!(=> "Expected an Integer, found a {:?}.", rhs),
            },
            Value::Single(lhs) => match rhs {
                Value::Single(rhs) => Ok(Value::Single(lhs + rhs)),
                _ => error!(=> "Expected a Single, found a {:?}.", rhs),
            },
            _ => error!(=> "Expected a Single or an Integer, found a {:?}.", lhs),
        }
    }
    pub fn sub(&mut self) -> Result<Value> {
        let rhs = self.pop()?;
        let lhs = self.pop()?;

        match lhs {
            Value::Integer(lhs) => match rhs {
                Value::Integer(rhs) => Ok(Value::Integer(lhs - rhs)),
                _ => error!(=> "Expected an Integer, found a {:?}.", rhs),
            },
            Value::Single(lhs) => match rhs {
                Value::Single(rhs) => Ok(Value::Single(lhs - rhs)),
                _ => error!(=> "Expected a Single, found a {:?}.", rhs),
            },
            _ => error!(=> "Expected a Single or an Integer, found a {:?}.", lhs),
        }
    }
    pub fn mul(&mut self) -> Result<Value> {
        let rhs = self.pop()?;
        let lhs = self.pop()?;

        match lhs {
            Value::Integer(lhs) => match rhs {
                Value::Integer(rhs) => Ok(Value::Integer(lhs * rhs)),
                _ => error!(=> "Expected an Integer, found a {:?}.", rhs),
            },
            Value::Single(lhs) => match rhs {
                Value::Single(rhs) => Ok(Value::Single(lhs * rhs)),
                _ => error!(=> "Expected a Single, found a {:?}.", rhs),
            },
            _ => error!(=> "Expected a Single or an Integer, found a {:?}.", lhs),
        }
    }
    pub fn div(&mut self) -> Result<Value> {
        let rhs = self.pop()?;
        let lhs = self.pop()?;

        match lhs {
            Value::Integer(lhs) => match rhs {
                Value::Integer(rhs) => Ok(Value::Integer(lhs / rhs)),
                _ => error!(=> "Expected an Integer, found a {:?}.", rhs),
            },
            Value::Single(lhs) => match rhs {
                Value::Single(rhs) => Ok(Value::Single(lhs / rhs)),
                _ => error!(=> "Expected a Single, found a {:?}.", rhs),
            },
            _ => error!(=> "Expected a Single or an Integer, found a {:?}.", lhs),
        }
    }



}
