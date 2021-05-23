use crate::{    
    bytecode::{Bytecode, BytecodePattern, Chunk, OpCode},
    error, bug,
    parser::Literal,
    OrionError, Result,
};

use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    Single(f32),
    String(String),
    Lambda(u16),
    Constructor(u16, Vec<Value>),
    Tuple(Vec<Value>),
    Initialzing,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Single(r) => write!(f, "{}{}", r, if r.fract() == 0.0 { "." } else { "" }),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Lambda(u) => write!(f, "\\{}", u),
            Value::Constructor(id, args) => write!(f, "#{}({})", id, args.into_iter().map(|a| format!("{}", a)).collect::<Vec<String>>().join(" ")),
            Value::Tuple(args) => write!(f, "({})", args.into_iter().map(|a| format!("{}", a)).collect::<Vec<String>>().join(" ")),
            _ => bug!("UNEXPECTED_INITIALIZING")
        }
    }
}

pub struct VM<const STACK_SIZE: usize> {
    pub input: Bytecode,
    pub stack: Vec<Value>,
    pub builtins: Vec<(
        fn(&mut VM<STACK_SIZE>, &mut Vec<Value>) -> Result<Value>,
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
                stack.push(Value::Tuple(vec![]));
                stack
            },
            builtins: vec![],
            ip: 0,
        };
        to_ret.register_builtin(Self::add, 2);
        to_ret.register_builtin(Self::dbg, 1);
        to_ret
    }

    fn dbg(&mut self, _: &mut Vec<Value>) -> Result<Value> {
        println!("{:?}", self.pop()?);
        Ok(Value::Tuple(vec![]))
    }
    fn add(&mut self, _: &mut Vec<Value>) -> Result<Value> {
        let lhs = self.pop()?;
        let rhs = self.pop()?;

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
    fn register_builtin(
        &mut self,
        func: fn(&mut VM<STACK_SIZE>, &mut Vec<Value>) -> Result<Value>,
        argc: u8,
        ) {
        self.builtins.push((func, argc))
    }
    fn pop(&mut self) -> Result<Value> {
        match self.stack.pop() {
            Some(v) => Ok(v),
            None => error!(=> "Stack underflow."),
        }
    }
    fn eval_opcode(&mut self, opcode: OpCode, ctx: &mut Vec<Value>) -> Result<()> {
        match opcode {
            OpCode::LoadConst(id) => self.stack.push(to_val(&self.input.constants[id as usize])),
            OpCode::LoadSym(id) => {
                self.stack.push(ctx[id as usize].clone())
            },
            OpCode::Def(sym_id, instr_length) => {
                if sym_id as usize >= ctx.len() {
                    ctx.push(Value::Initialzing);
                } else {
                    ctx[sym_id as usize] = Value::Initialzing;
                }
                (1..=instr_length).map(|i| {
                    let instr = self.input.instructions[self.ip + i as usize];
                    self.eval_opcode(instr,ctx)
                }).collect::<Result<()>>()?;
                self.ip += instr_length as usize;
                let popped = self.pop()?;
                ctx[sym_id as usize] = popped;
            }
            OpCode::Lambda(chunk_id) => self.stack.push(Value::Lambda(chunk_id)),
            OpCode::Call(argc) => {
                let mut args = vec![];
                for _ in 0..argc {
                    args.push(self.pop()?);
                }
                args.reverse();
                let func = self.pop()?;
                if let Value::Lambda(chunk) = func {
                    let chunk = &self.input.chunks[chunk as usize];
                    if chunk.reference.len() != args.len() {
                        return error!(
                            => "Expected {} arguments, found {}.",
                            chunk.reference.len(),
                            args.len()
                            );
                    }
                    let prev_ctx = ctx.clone(); // Save symbol table before editing.
                    for idx in 0..chunk.reference.len() {
                        // Fetch arguments and replace the symbol table.
                        let val = args[idx].clone();
                        let chunk_id = chunk.reference[idx] as usize;
                        if chunk_id >= ctx.len() {
                            ctx.push(val); // Push if symbol has not been affected yet.
                        } else {
                            ctx[chunk_id] = val;
                        }
                    }

                    for instr in chunk.instructions.clone() {
                        self.eval_opcode(instr, ctx)?; // Eval chunk body.
                    }

                    *ctx = prev_ctx; // Drop modified context with replaced arguments and reuse older context.
                } else {
                    return error!(=> "Expected a Lambda, found a {:?}.", func);
                }
            }
            OpCode::Builtin(idx, argc) => {
                let (f, f_argc) = self.builtins[idx as usize];
                if f_argc != argc {
                    return error!(
                        => "Builtin 0x{:02x} takes {} arguments, but {} arguments were supplied.",
                        idx, f_argc, argc
                        );
                }
                let to_push = f(self, ctx)?;
                self.stack.push(to_push);
            }
            OpCode::Constructor(idx, to_eval) => {
                let amount = self.input.constructors[idx as usize];
                for _ in 1..=amount {
                    self.ip += 1;
                    let instruction = self.input.instructions[self.ip];
                    self.eval_opcode(instruction, ctx)?;
                }
                let mut vals = (0..to_eval)
                    .map(|_| self.pop())
                    .collect::<Result<Vec<Value>>>()?;
                vals.reverse();
                self.stack.push(Value::Constructor(idx, vals));
            }
            OpCode::Tuple(to_eval) => {
                for _ in 1..=to_eval{
                    self.ip += 1;
                    let instruction = self.input.instructions[self.ip];
                    self.eval_opcode(instruction, ctx)?;
                }
                let mut vals = (0..to_eval)
                    .map(|_| self.pop())
                    .collect::<Result<Vec<Value>>>()?;
                vals.reverse();
                self.stack.push(Value::Tuple(vals))
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

                for plausible in plausible {
                    match self.match_and_bound(&to_match, plausible.0) {
                        Some(to_bind) => {
                            let mut new_ctx = ctx.clone();
                            let mut new_stack = (0..to_bind.len()).map(|_| self.pop()).rev().collect::<Result<Vec<_>>>()?;
                            to_bind.into_iter().map(|sym_id| {
                                let val = new_stack.pop().unwrap();
                                if sym_id as usize >= new_ctx.len() {
                                    new_ctx.push(val);
                                } else {
                                    new_ctx[sym_id as usize] = val;
                                }
                                Ok(())
                            }).collect::<Result<Vec<_>>>()?;
                            plausible.1.into_iter().map(|instr| {
                                self.eval_opcode(instr, &mut new_ctx)
                            }).collect::<Result<Vec<_>>>()?;
                        },
                        None => {},
                    }
                }
            }
        }

        Ok(())
    }
    fn match_and_bound(&mut self, val: &Value, pat_idx: u16) -> Option<Vec<u16>> {
        let pat = &self.input.patterns[pat_idx as usize];
        match pat {
            BytecodePattern::Elide => Some(vec![]),
            BytecodePattern::Var(idx) => {
                self.stack.push(val.clone());
                Some(vec![*idx])
            }
            BytecodePattern::Literal(idx) => {
                let lit = &self.input.constants[*idx as usize];
                match val {
                    Value::Integer(lhs) => match lit {
                        Literal::Integer(rhs) => if lhs == rhs { Some(vec![]) } else { None },
                        _ => None,
                    }
                    Value::Single(lhs) => match lit {
                        Literal::Single(rhs) => if lhs == rhs { Some(vec![]) } else { None },
                        _ => None,
                    }
                    Value::String(lhs) => match lit {
                        Literal::String(rhs) => if lhs == rhs { Some(vec![]) } else { None },
                        _ => None
                    }
                    _ => bug!("FAILED_PLAUSIBLE_UNEXPECTED_VALUE")
                }
            }
            BytecodePattern::Tuple(pats) => if let Value::Tuple(vals) = val {
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
            BytecodePattern::Constr(idx, pats) => if let Value::Constructor(to_match_idx, vals) = val {
                if to_match_idx == idx {
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
        match self.input.patterns[pat as usize] {
            BytecodePattern::Var(_) | BytecodePattern::Elide => true,
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
    pub fn eval(&mut self, ctx: Vec<Value>) -> Result<Vec<Value>> {
        let mut ctx = ctx;
        while self.ip < self.input.instructions.len() {
            let instruction = self.input.instructions[self.ip];
            self.eval_opcode(instruction, &mut ctx)?;
            self.ip += 1;
        }
        Ok(ctx)
    }
}
