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
use crate::{    
    bytecode::{Bytecode, BytecodePattern, OpCode},
    error, bug,
    parser::Literal,
    Result,
};

use std::{fmt::{self, Display, Formatter}, rc::Rc};

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    Single(f32),
    String(String),
    Lambda(u16),
    Constructor(u16, Vec<Rc<Value>>),
    Tuple(Vec<Rc<Value>>),
    Initialzing,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Single(r) => write!(f, "{}{}", r, if r.fract() == 0.0 { "." } else { "" }),
            Value::String(s) => write!(f, "{}", s),
            Value::Lambda(u) => write!(f, "\\{}", u),
            Value::Constructor(id, args) => write!(f, "#{}({})", id, args.into_iter().map(|a| format!("{}", a)).collect::<Vec<String>>().join(" ")),
            Value::Tuple(args) => write!(f, "({})", args.into_iter().map(|a| format!("{}", a)).collect::<Vec<String>>().join(" ")),
            _ => bug!("UNEXPECTED_INITIALIZING")
        }
    }
}
pub struct VM<const STACK_SIZE: usize> {
    pub input: Bytecode,
    pub stack: Vec<Rc<Value>>,
    pub builtins: Vec<(
        fn(&mut VM<STACK_SIZE>) -> Result<Rc<Value>>,
        u8,
        )>,
        pub ip: usize,
}
fn to_val(lit: &Literal) -> Value {
    match lit {
        Literal::Integer(i) => Value::Integer(*i),
        Literal::Single(s) => Value::Single(*s),
        Literal::String(s) => Value::String(s.to_string()),
    }
}
impl<const STACK_SIZE: usize> VM<STACK_SIZE> {
    pub fn new(input: Bytecode) -> Self {
        let mut to_ret = Self {
            input,
            stack: {
                let mut stack = Vec::with_capacity(STACK_SIZE);
                stack.push(Rc::new(Value::Tuple(vec![])));
                stack
            },
            builtins: vec![],
            ip: 0,
        };
        to_ret.register_builtin(Self::add, 2);
        to_ret.register_builtin(Self::sub, 2);
        to_ret.register_builtin(Self::mul, 2);
        to_ret.register_builtin(Self::div, 2);
        to_ret.register_builtin(Self::neg, 1);
        to_ret.register_builtin(Self::cos, 1);
        to_ret.register_builtin(Self::sin, 1);
        to_ret.register_builtin(Self::tan, 1);
        to_ret.register_builtin(Self::acos, 1);
        to_ret.register_builtin(Self::asin, 1);
        to_ret.register_builtin(Self::atan, 1);

        to_ret.register_builtin(Self::format, 2);

        to_ret.register_builtin(Self::put_str, 1);
        to_ret.register_builtin(Self::get_line, 0);

        to_ret.register_builtin(Self::r#type, 1);
        to_ret.register_builtin(Self::cmp, 1);
        to_ret
    }
    fn _cmp(&mut self, lhs: &Value, rhs: &Value) -> Result<std::cmp::Ordering> {
        use std::cmp::Ordering;
        match lhs {
            Value::Single(lhs) => match rhs {
                Value::Single(rhs) => {
                    Ok(lhs.partial_cmp(&rhs).unwrap())
                }
                _ => error!(=> "Expected a Single, found a {}.", self.val_type(&rhs)?),
            }

            Value::Integer(lhs) => match rhs {
                Value::Integer(rhs) => {
                    Ok(lhs.cmp(&rhs))
                }
                _ => error!(=> "Expected an Integer, found a {}.", self.val_type(&rhs)?),
            }
            Value::String(lhs) => match rhs {
                Value::String(rhs) => {
                    Ok(lhs.cmp(&rhs))
                }
                _ => error!(=> "Expected a String, found a {}.", self.val_type(&rhs)?),
            }
            Value::Constructor(lid, vlhs) => match &rhs {
                Value::Constructor(rid, vrhs) => {
                    let tlhs = self.val_type(&lhs)?;
                    let trhs = self.val_type(&rhs)?;
                    if tlhs != trhs {
                        error!(=> "Expected a {}, found a {}.", tlhs, trhs)
                    } else {
                        if lid != rid {
                            error!(=> "Not the same enum variants, expected 0x{:04x}, found 0x{:04x}", lid, rid)
                        } else {
                            let mut to_ret = Ordering::Equal;

                            for idx in 0..vlhs.len() {
                                let lhs = &vlhs[idx];
                                let rhs = &vrhs[idx];
                                let res = self._cmp(lhs, rhs)?;
                                if res != Ordering::Equal {
                                    to_ret = res;
                                    break;
                                }
                            }
                            Ok(to_ret)
                        }
                    }
                }
                _ => error!(=> "Expected a Constructor, found a {}.", self.val_type(&rhs)?),
            }
            Value::Tuple(vlhs) => match rhs {
                Value::Tuple(vrhs) => {
                    let tlhs = self.val_type(&lhs)?;
                    let trhs = self.val_type(&rhs)?;
                    if tlhs != trhs {
                        error!(=> "Expected a {}, found a {}.", tlhs, trhs)
                    } else {
                        let mut to_ret = Ordering::Equal;
                        for idx in 0..vlhs.len() {
                            let lhs = &vlhs[idx];
                            let rhs = &vrhs[idx];
                            let res = self._cmp(lhs, rhs)?;
                            if res != Ordering::Equal {
                                to_ret = res; 
                                break;
                            }
                        }
                        Ok(to_ret)
                    }
                }
                _ => error!(=> "Expected a Tuple, found a {}.", self.val_type(&rhs)?),
            }
            _ => error!(=> "Expected a String, found a {}.", self.val_type(&rhs)?),
        }
    }

    fn cmp(&mut self) -> Result<Rc<Value>> {
        use std::cmp::Ordering;
        let rhs = self.pop()?;
        let lhs = self.pop()?;
        let correspondance = [Ordering::Less, Ordering::Equal, Ordering::Greater];
        let res = self._cmp(&lhs, &rhs)?;
        Ok(Rc::new(Value::Integer(correspondance.iter().position(|x| x == &res).unwrap() as i32)))
    }
    fn r#type(&mut self) -> Result<Rc<Value>> {
        let popped = self.pop()?;
        let to_ret = Ok(Rc::new(Value::String(self.val_type(&popped)?)));
        self.stack.push(popped);
        to_ret
    }
    pub fn val_type(&mut self, popped: &Value) -> Result<String> {
        let to_ret = Ok(match popped {
            Value::Constructor(idx, _) => self.input.types[self.input.types.iter().position(|(_, start, end)| (start..=end).contains(&&idx)).unwrap()].0.clone(),
            Value::Tuple(content) => format!("({})", content.iter().map(|v|{
                let to_ret = self.val_type(v)?;
                Ok(to_ret)
            }).collect::<Result<Vec<String>>>()?.join(" ")),
            Value::String(_) => "String".to_string(),
            Value::Single(_) => "Single".to_string(),
            Value::Integer(_) => "Integer".to_string(),
            Value::Lambda(_) => "Lambda".to_string(),
            Value::Initialzing => bug!("UNEXPECTED_INITALZING"),
        });
        to_ret
    }
    fn register_builtin(
        &mut self,
        func: fn(&mut VM<STACK_SIZE>) -> Result<Rc<Value>>,
        argc: u8,
        ) {
        self.builtins.push((func, argc))
    }
    pub fn pop(&mut self) -> Result<Rc<Value>> {
        match self.stack.pop() {
            Some(v) => Ok(v),
            None => error!(=> "Stack underflow."),
        }

    }
    fn decl(&mut self, sym_id: u16, val: Rc<Value>, ctx: &mut Vec<Rc<Value>>, sym_ref: &mut Vec<u16>) {
        let id = if !sym_ref.contains(&sym_id) {
            sym_ref.push(sym_id);
            sym_ref.len()
        } else {
            sym_ref.iter().position(|id| id == &sym_id).unwrap()
        };
        if id >= ctx.len() {
            ctx.push(val);
        } else {
            ctx[id] = val;
        }
    }
    fn eval_opcode(&mut self, opcode: OpCode, ctx: &mut Vec<Rc<Value>>, sym_ref: &mut Vec<u16>, instructions: &[OpCode]) -> Result<()> {
        match opcode {
            OpCode::Panic(file, line) => if let Literal::Integer(line) = self.input.constants[line as usize] {
                if let Literal::String(file) = self.input.constants[file as usize].clone() {
                    let popped = self.pop()?;
                    eprintln!("Program panicked at '{}', {}:{}.", popped, file, line);
                    std::process::exit(1);
                }
            }
            OpCode::LoadConst(id) => self.stack.push(Rc::new(to_val(&self.input.constants[id as usize]))),
            OpCode::LoadSym(id) => {
                let local_id = if !sym_ref.contains(&id) {
                    error!(=> "Unbound function: {}.", self.input.symbols[id as usize])
                } else {
                    Ok(sym_ref.iter().position(|sid| sid == &id).unwrap())                
                }?;
                self.stack.push(ctx[local_id as usize].clone())
            },
            OpCode::Def(sym_id, instr_length) => {
                let id = if !sym_ref.contains(&sym_id) {
                    sym_ref.push(sym_id);
                    sym_ref.len() - 1
                } else {
                    sym_ref.iter().position(|id| id == &sym_id).unwrap()
                };
                if id >= ctx.len() {
                    ctx.push(Rc::new(Value::Initialzing));
                } else {
                    ctx[id] = Rc::new(Value::Initialzing);
                }
                (0..instr_length).map(|_| {
                    self.ip += 1;
                    let instr = instructions[self.ip];
                    self.eval_opcode(instr, ctx, sym_ref, &instructions)
                }).collect::<Result<()>>()?;
                let popped = self.pop()?;
                ctx[id] = popped;
            }
            OpCode::Lambda(chunk_id) => self.stack.push(Rc::new(Value::Lambda(chunk_id))),
            OpCode::Call(argc) => {
                let mut args = vec![];
                for _ in 0..argc {
                    args.push(self.pop()?);
                }
                args.reverse();
                let func = self.pop()?;
                if let Value::Lambda(chunk) = *func {
                    let chunk = self.input.chunks[chunk as usize].clone();
                    if chunk.reference.len() != args.len() {
                        return error!(
                            => "Expected {} arguments, found {}.",
                            chunk.reference.len(),
                            args.len()
                            );
                    }
                    let prev_ctx = ctx.clone(); // Save symbol table before editing.
                    let prev_ref = sym_ref.clone();
                    for idx in 0..chunk.reference.len() {
                        // Fetch arguments and replace the symbol table.
                        let val = args[idx].clone();
                        let sym_id = chunk.reference[idx];
                        self.decl(sym_id, val, ctx, sym_ref);
                    }
                    let prev_ip = self.ip;
                    self.ip = 0; // Reset the instruction counter to fit chunk instructions
                    while self.ip < chunk.instructions.len() {
                        let instr = chunk.instructions[self.ip];
                        self.eval_opcode(instr, ctx, sym_ref, &chunk.instructions)?; // Eval chunk body.
                        self.ip += 1;
                    }
                    self.ip = prev_ip;

                    *ctx = prev_ctx; // Drop modified context with replaced arguments and reuse older context.
                    *sym_ref = prev_ref;
                } else {
                    return error!(=> "Expected a Lambda, found a {:?}.", func);
                }
            }
            OpCode::Builtin(idx, argc) => {
                let (f, f_argc) = self.builtins[idx as usize];
                if f_argc != argc as u8 {
                    return error!(
                        => "Builtin 0x{:02x} takes {} arguments, but {} arguments were supplied.",
                        idx, f_argc, argc
                        );
                }
                let to_push = f(self)?;
                self.stack.push(to_push);
            }
            OpCode::Constructor(idx, to_eval) => {
                let amount = self.input.constructors[idx as usize];
                for _ in 1..=amount {
                    self.ip += 1;
                    let instruction = instructions[self.ip].clone();
                    self.eval_opcode(instruction, ctx, sym_ref, instructions.clone())?;
                }
                let mut vals = (0..to_eval)
                    .map(|_| self.pop())
                    .collect::<Result<Vec<Rc<Value>>>>()?;
                vals.reverse();
                self.stack.push(Rc::new(Value::Constructor(idx, vals)));
            }
            OpCode::Tuple(to_eval) => {
                (0..to_eval).map(|_| {
                    self.ip += 1;
                    let instr = instructions[self.ip];
                    self.eval_opcode(instr, ctx, sym_ref, instructions)
                }).collect::<Result<Vec<_>>>()?;
                let mut vals = (0..to_eval)
                    .map(|_| self.pop())
                    .collect::<Result<Vec<Rc<Value>>>>()?;
                vals.reverse();
                self.stack.push(Rc::new(Value::Tuple(vals)));
            }
            OpCode::Match(idx) => {
                let to_match = self.pop()?;
                let patterns = self.input.matches[idx as usize].clone();
                let plausible = patterns.into_iter().map(|(pat, to_exec)| {
                    if self.is_plausible(pat, &to_match) {
                        Some((pat, to_exec))
                    } else {
                        None
                    }
                }).filter(|p| !p.is_none()).map(|p| p.unwrap()).collect::<Vec<(u16, Vec<OpCode>)>>();
                for plausible in plausible.into_iter() {
                    match self.match_and_bound(&to_match, plausible.0) {
                        Some(to_bind) => {
                            let mut new_ctx = ctx.clone();
                            let mut new_ref = sym_ref.clone();
                            let mut new_stack = (0..to_bind.len()).map(|_| self.pop()).rev().collect::<Result<Vec<_>>>()?;
                            to_bind.into_iter().for_each(|sym_id| {
                                let val = new_stack.pop().unwrap();
                                self.decl(sym_id, val, &mut new_ctx, &mut new_ref);    
                            });
                            let saved = self.ip;
                            self.ip = 0;
                            plausible.1.clone().into_iter().map(|instr| {
                                self.eval_opcode(instr, &mut new_ctx, &mut new_ref, &plausible.1)
                            }).collect::<Result<Vec<_>>>()?;
                            self.ip = saved;
                            return Ok(())
                        },
                        None => {},
                    }
                }
                return error!(=> "No pattern to be matched.");
            }
        }

        Ok(())
    }
    fn match_and_bound(&mut self, val: &Rc<Value>, pat_idx: u16) -> Option<Vec<u16>> {
        let pat = &self.input.patterns[pat_idx as usize];
        match pat {
            BytecodePattern::Any => Some(vec![]),
            BytecodePattern::Var(idx) => {
                self.stack.push((*val).clone());
                Some(vec![*idx])
            }
            BytecodePattern::Literal(idx) => {
                let lit = &self.input.constants[*idx as usize];
                match (**val).clone() {
                    Value::Integer(lhs) => match lit {
                        Literal::Integer(rhs) => if lhs == *rhs { Some(vec![]) } else { None },
                        _ => None,
                    }
                    Value::Single(lhs) => match lit {
                        Literal::Single(rhs) => if lhs == *rhs { Some(vec![]) } else { None },
                        _ => None,
                    }
                    Value::String(lhs) => match lit {
                        Literal::String(rhs) => if lhs == *rhs { Some(vec![]) } else { None },
                        _ => None
                    }
                    _ => bug!("FAILED_PLAUSIBLE_UNEXPECTED_VALUE")
                }
            }
            BytecodePattern::Tuple(pats) => if let Value::Tuple(vals) = (**val).clone() {
                let pats = pats.clone();
                if vals.len() == pats.len() {
                    let mut to_ret = vec![];
                    for i in 0..vals.len() {
                        if !self.is_plausible(pats[i], &vals[i]) {
                            return None;
                        } else {
                            match self.match_and_bound(&vals[i], pats[i]) {
                                Some(ctx) => to_ret.extend(ctx),
                                None => return None,
                            }
                        }
                    }
                    Some(to_ret)
                } else {
                    None
                }
            } else {
                None
            }
            BytecodePattern::Constr(idx, pats) => if let Value::Constructor(to_match_idx, vals) = (**val).clone() {
                if to_match_idx == *idx {
                    let pats = pats.clone();
                    let mut to_ret = vec![];
                    for i in 0..vals.len() {
                        if !self.is_plausible(pats[i], &vals[i]) {
                            return None;
                        } else {
                            match self.match_and_bound(&vals[i], pats[i]) {
                                Some(ctx) => to_ret.extend(ctx),
                                None => return None,
                            }
                        }
                    }
                    Some(to_ret)
                } else {
                    None
                }
            } else {
                None
            },
        }
    }
    fn is_plausible(&self, pat: u16, to_match: &Value) -> bool {
        let pat = self.input.patterns[pat as usize].clone();
        match pat {
            BytecodePattern::Var(_) | BytecodePattern::Any => true,
            BytecodePattern::Constr(_, _) => if let Value::Constructor(_, _) = to_match {
                true
            } else {
                false
            }
            BytecodePattern::Tuple(_) => if let Value::Tuple(_) = to_match {
                true
            } else {
                false
            }
            BytecodePattern::Literal(lid) => match &self.input.constants[lid as usize] {
                Literal::Integer(_) => match to_match {
                    Value::Integer(_) => true,
                    _ => false,
                }
                Literal::Single(_) => match to_match {
                    Value::Single(_) => true,
                    _ => false
                }
                Literal::String(_) => match to_match {
                    Value::String(_) => true,
                    _ => false,
                }
            }
        }
    }
    pub fn eval(&mut self, mut sym_ref: Vec<u16>, mut ctx: Vec<Rc<Value>>) -> Result<(Vec<Rc<Value>>, Vec<u16>)> {
        while self.ip < self.input.instructions.len() {
            let instruction = self.input.instructions[self.ip];
            let instrs = self.input.instructions.clone();
            self.eval_opcode(instruction, &mut ctx, &mut sym_ref, &instrs)?;
            self.ip += 1;
        }
        Ok((ctx, sym_ref))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::compiler::Compiler;
    use crate::cli::compile_dbg;
    use std::time::Instant;

    #[cfg(not(debug_assertions))] // Run only in Release
    #[test]
    fn ackermann33() -> Result<()> {
        let tokens = Lexer::new("(def ack (λ (m n)
        (match (, m n)
         ((, 0 _) (+ n 1))
         ((, _ 0) (ack (- m 1) 1))
         (_ (ack (- m 1) (ack m (- n 1)))))))", "TEST").proc_tokens()?;
        let ast = Parser::new(tokens, "TEST").parse()?;
        let (bytecode, symbols, _) = Compiler::new(ast, "TEST", Bytecode::new(), vec![], true, "".to_string())?.compile(vec![])?;

        let (ctx, sym_ref) = VM::<256>::new(bytecode.clone()).eval(vec![], vec![])?;
        let (call_bytecode, ..) = Compiler::new(Parser::new(Lexer::new("(ack 3 3)", "TEST").proc_tokens()?, "TEST").parse()?, "TEST", bytecode, vec![], true, "".to_string())?.compile(symbols)?;
        let mut vals = (0..1000).map(|_| {
            let mut vm = VM::<256>::new(call_bytecode.clone());
            let start = Instant::now();
            vm.eval(sym_ref.clone(), ctx.clone())?;
            let elapsed = start.elapsed();
            Ok(elapsed.as_millis() as u32)
        }).collect::<Result<Vec<u32>>>()?;
        vals.sort();
        let total = vals.iter().sum::<u32>() as f32;
        let average = total / vals.len() as f32;
        let stddev = (0..vals.len()).into_iter().map(|i| {
            (vals[i] as f32 - average).powi(2)
        }).sum::<f32>().sqrt();
        println!("Total: {}ms ; Average: {}ms ; Median: {}ms ; Amplitude: {}ms ; Stddev: {}us", total, average, vals[vals.len() / 2], vals[vals.len() - 1] - vals[0], stddev);
        Ok(())
    }
}
