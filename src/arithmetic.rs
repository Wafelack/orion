/*
 *  Copyright (C) 2021, Wafelack <wafelack@protonmail.com>
 *
 *  ------------------------------------------------------
 *
 *     This file is part of Orion.
 *
 *  Orion is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Orion is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Orion.  If not, see <https://www.gnu.org/licenses/>.
 */
use std::rc::Rc;
use crate::{vm::{VM, Value}, error, Result};

impl<const STACK_SIZE: usize> VM<STACK_SIZE> {
    pub fn add(&mut self) -> Result<Rc<Value>> {
        let rhs = self.pop()?;
        let lhs = self.pop()?;

        match *lhs {
            Value::Integer(lhs) => match *rhs {
                Value::Integer(rhs) => Ok(Rc::new(Value::Integer(lhs + rhs))),
                _ => error!(=> "Expected an Integer, found a {}.", self.val_type(&rhs)?),
            },
            Value::Single(lhs) => match *rhs {
                Value::Single(rhs) => Ok(Rc::new(Value::Single(lhs + rhs))),
                _ => error!(=> "Expected a Single, found a {}.", self.val_type(&rhs)?),
            },
            _ => error!(=> "Expected a Single or an Integer, found a {}.", self.val_type(&*lhs)?),
        }
    }
    pub fn sub(&mut self) -> Result<Rc<Value>> {
        let rhs = self.pop()?;
        let lhs = self.pop()?;

        match *lhs {
            Value::Integer(lhs) => match *rhs {
                Value::Integer(rhs) => Ok(Rc::new(Value::Integer(lhs - rhs))),
                _ => error!(=> "Expected an Integer, found a {}.", self.val_type(&rhs)?),
            },
            Value::Single(lhs) => match *rhs {
                Value::Single(rhs) => Ok(Rc::new(Value::Single(lhs - rhs))),
                _ => error!(=> "Expected a Single, found a {}.", self.val_type(&rhs)?),
            },
            _ => error!(=> "Expected a Single or an Integer, found a {}.", self.val_type(&lhs)?),
        }
    }
    pub fn mul(&mut self) -> Result<Rc<Value>> {
        let rhs = self.pop()?;
        let lhs = self.pop()?;

        match *lhs {
            Value::Integer(lhs) => match *rhs {
                Value::Integer(rhs) => Ok(Rc::new(Value::Integer(lhs * rhs))),
                _ => error!(=> "Expected an Integer, found a {}.", self.val_type(&rhs)?),
            },
            Value::Single(lhs) => match *rhs {
                Value::Single(rhs) => Ok(Rc::new(Value::Single(lhs * rhs))),
                _ => error!(=> "Expected a Single, found a {}.", self.val_type(&rhs)?),
            },
            _ => error!(=> "Expected a Single or an Integer, found a {}.", self.val_type(&lhs)?),
        }
    }
    pub fn div(&mut self) -> Result<Rc<Value>> {
        let rhs = self.pop()?;
        let lhs = self.pop()?;

        match *lhs {
            Value::Integer(lhs) => match *rhs {
                Value::Integer(rhs) => Ok(Rc::new(Value::Integer(lhs / rhs))),
                _ => error!(=> "Expected an Integer, found a {}.", self.val_type(&rhs)?),
            },
            Value::Single(lhs) => match *rhs {
                Value::Single(rhs) => Ok(Rc::new(Value::Single(lhs / rhs))),
                _ => error!(=> "Expected a Single, found a {}.", self.val_type(&rhs)?),
            },
            _ => error!(=> "Expected a Single or an Integer, found a {}.", self.val_type(&*lhs)?),
        }
    }
    pub fn neg(&mut self) -> Result<Rc<Value>> {
        let val = self.pop()?;

        match *val {
            Value::Integer(val) => Ok(Rc::new(Value::Integer(-val))),
            Value::Single(val) => Ok(Rc::new(Value::Single(-val))),
            _ => error!(=> "Expected a Single or an Integer, found a {}.", self.val_type(&*val)?),
        }
    }
    pub fn cos(&mut self) -> Result<Rc<Value>> {
        let val = self.pop()?;

        match *val {
            Value::Single(val) => Ok(Rc::new(Value::Single(val.cos()))),
            _ => error!(=> "Expected a Single, found a {}.", self.val_type(&*val)?),
        }
    }
    pub fn sin(&mut self) -> Result<Rc<Value>> {
        let val = self.pop()?;

        match *val {
            Value::Single(val) => Ok(Rc::new(Value::Single(val.sin()))),
            _ => error!(=> "Expected a Single, found a {}.", self.val_type(&*val)?),
        }
    }
    pub fn tan(&mut self) -> Result<Rc<Value>> {
        let val = self.pop()?;

        match *val {
            Value::Single(val) => Ok(Rc::new(Value::Single(val.tan()))),
            _ => error!(=> "Expected a Single, found a {}.", self.val_type(&*val)?),
        }
    }
    pub fn acos(&mut self) -> Result<Rc<Value>> {
        let val = self.pop()?;

        match *val {
            Value::Single(val) => Ok(Rc::new(Value::Single(val.acos()))),
            _ => error!(=> "Expected a Single, found a {}.", self.val_type(&*val)?),
        }
    }
    pub fn asin(&mut self) -> Result<Rc<Value>> {
        let val = self.pop()?;

        match *val {
            Value::Single(val) => Ok(Rc::new(Value::Single(val.asin()))),
            _ => error!(=> "Expected a Single, found a {}.", self.val_type(&*val)?),
        }
    }
    pub fn atan(&mut self) -> Result<Rc<Value>> {
        let val = self.pop()?;

        match *val {
            Value::Single(val) => Ok(Rc::new(Value::Single(val.atan()))),
            _ => error!(=> "Expected a Single, found a {}.", self.val_type(&*val)?),
        }
    }
}
