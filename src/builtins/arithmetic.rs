/*
 *  Copyright (C) 2021, Wafelack <wafelack@protonmail.com>
 *
 *  ------------------------------------------------------
 *
 *     This file is part of Orion.
 *
 *  Orion is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *  Orion is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Orion.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::{interpreter::{Interpreter, Value}, OrionError, error, Result};

impl Interpreter {
    pub fn add(&mut self, args: Vec<Value>) -> Result<Value> {
        if let Value::Single(mut x0) = args[0] {
           for elem in args.into_iter().skip(1) {
                if let Value::Single(x) = elem {
                    x0 += x;
                } else {
                    return error!("Expected a Single, found a {}.", self.get_val_type(&elem));
                }
            }

            Ok(Value::Single(x0))
        } else if let Value::Integer(mut z0) = args[0] { 
            for elem in args.into_iter().skip(1) {
                if let Value::Integer(z) = elem {
                    z0 += z;
                } else {
                    return error!("Expected a Single, found a {}.", self.get_val_type(&elem));
                }
            }

            Ok(Value::Integer(z0))
        } else {
            error!("Expected a Single or an Integer, found a {}.", self.get_val_type(&args[0]))
        }
    }

    pub fn sub(&mut self, args: Vec<Value>) -> Result<Value> {
        if let Value::Single(mut x0) = args[0] {
           for elem in args.into_iter().skip(1) {
                if let Value::Single(x) = elem {
                    x0 -= x;
                } else {
                    return error!("Expected a Single, found a {}.", self.get_val_type(&elem));
                }
            }

            Ok(Value::Single(x0))
        } else if let Value::Integer(mut z0) = args[0] { 
            for elem in args.into_iter().skip(1) {
                if let Value::Integer(z) = elem {
                    z0 -= z;
                } else {
                    return error!("Expected a Single, found a {}.", self.get_val_type(&elem));
                }
            }

            Ok(Value::Integer(z0))
        } else {
            error!("Expected a Single or an Integer, found a {}.", self.get_val_type(&args[0]))
        }
    }

    pub fn mul(&mut self, args: Vec<Value>) -> Result<Value> {
        if let Value::Single(mut x0) = args[0] {
            for elem in args.into_iter().skip(1) {
                if let Value::Single(x) = elem {
                    x0 *= x;
                } else {
                    return error!("Expected a Single, found a {}.", self.get_val_type(&elem));
                }
            }

            Ok(Value::Single(x0))
        } else if let Value::Integer(mut z0) = args[0] { 
            for elem in args.into_iter().skip(1) {
                if let Value::Integer(z) = elem {
                    z0 *= z;
                } else {
                    return error!("Expected a Single, found a {}.", self.get_val_type(&elem));
                }
            }

            Ok(Value::Integer(z0))
        } else {
            error!("Expected a Single or an Integer, found a {}.", self.get_val_type(&args[0]))
        }
    }

    pub fn div(&mut self, args: Vec<Value>) -> Result<Value> {
        if let Value::Single(mut x0) = args[0] {
            for elem in args.into_iter().skip(1) {
                if let Value::Single(x) = elem {
                    x0 /= x;
                } else {
                    return error!("Expected a Single, found a {}.", self.get_val_type(&elem));
                }
            }

            Ok(Value::Single(x0))
        } else if let Value::Integer(mut z0) = args[0] { 
            for elem in args.into_iter().skip(1) {
                if let Value::Integer(z) = elem {
                    z0 /= z;
                } else {
                    return error!("Expected a Single, found a {}.", self.get_val_type(&elem));
                }
            }

            Ok(Value::Integer(z0))
        } else {
            error!("Expected a Single or an Integer, found a {}.", self.get_val_type(&args[0]))
        }
    }
}
