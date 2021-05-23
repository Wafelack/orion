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
        fn(&mut VM<STACK_SIZE>) -> Result<Value>,
        i16,
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
        to_ret.register_builtin(Self::sub, 2);
        to_ret.register_builtin(Self::mul, 2);
        to_ret.register_builtin(Self::div, 2);
        to_ret.register_builtin(Self::dbg, 1);
        to_ret
    }
    fn dbg(&mut self) -> Result<Value> {
        println!("{:?}", self.pop()?);
        Ok(Value::Tuple(vec![]))
    }
    fn register_builtin(
        &mut self,
        func: fn(&mut VM<STACK_SIZE>) -> Result<Value>,
        argc: i16,
        ) {
        self.builtins.push((func, argc))
    }
    pub fn pop(&mut self) -> Result<Value> {
        match self.stack.pop() {
            Some(v) => Ok(v),
            None => error!(=> "Stack underflow."),
        }
    }
    fn eval_opcode(&mut self, opcode: OpCode, ctx: &mut Vec<Value>, instructions: Vec<OpCode>) -> Result<()> {
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
                    let instr = instructions[self.ip + i as usize];
                    self.eval_opcode(instr, ctx, instructions.clone())
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
                    let chunk = self.input.chunks[chunk as usize].clone();
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
                    let prev_ip = self.ip;
                    self.ip = 0; // Reset the instruction counter to fit chunk instructions
                    for instr in chunk.instructions.clone() {
                        self.eval_opcode(instr, ctx, chunk.instructions.clone())?; // Eval chunk body.
                        self.ip += 1;
                    }
                    self.ip = prev_ip;

                    *ctx = prev_ctx; // Drop modified context with replaced arguments and reuse older context.
                } else {
                    return error!(=> "Expected a Lambda, found a {:?}.", func);
                }
            }
            OpCode::Builtin(idx, argc) => {
                let (f, f_argc) = self.builtins[idx as usize];
                if f_argc != argc as i16 && f_argc != -1 {
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
                    self.eval_opcode(instruction, ctx, instructions.clone())?;
                }
                let mut vals = (0..to_eval)
                    .map(|_| self.pop())
                    .collect::<Result<Vec<Value>>>()?;
                vals.reverse();
                self.stack.push(Value::Constructor(idx, vals));
            }
            OpCode::Tuple(to_eval) => {
                for _ in 0..to_eval{
                    self.ip += 1;
                    let instruction = instructions[self.ip].clone();
                    println!("{:?}", instruction);
                    self.eval_opcode(instruction, ctx, instructions.clone())?;
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
                            plausible.1.clone().into_iter().map(|instr| {
                                self.eval_opcode(instr, &mut new_ctx, plausible.1.clone())
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
            self.eval_opcode(instruction, &mut ctx, self.input.instructions.clone())?;
            self.ip += 1;
        }
        Ok(ctx)
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

    #[test]
    fn ackermann() -> Result<()> {
        let tokens = Lexer::new("(def ack (λ (m n)
        (match (, m n)
         ((, 0 _) (+ n 1))
         ((, _ 0) (ack (- m 1) 1))
         (_ (ack (- m 1) (ack m (- n 1)))))))", "TEST").proc_tokens()?;
        let ast = Parser::new(tokens, "TEST").parse()?;
        let (bytecode, symbols) = Compiler::new(ast.clone(), "TEST", Bytecode::new()).compile(vec![])?;
        let (bytecode, symbols) = compile_dbg("TEST", ast, 3, vec![], Bytecode::new())?;
        
        let ctx = VM::<256>::new(bytecode.clone()).eval(vec![])?;
        let (call_bytecode, _) = Compiler::new(Parser::new(Lexer::new("(ack 3 3)", "TEST").proc_tokens()?, "TEST").parse()?, "TEST", bytecode).compile(symbols)?;
        let mut vals = (0..1000).map(|_| {
            let mut vm = VM::<256>::new(call_bytecode.clone());
            let start = Instant::now();
            vm.eval(ctx.clone())?;
            let elapsed = start.elapsed();
            Ok(elapsed.as_millis() as u32)
        }).collect::<Result<Vec<u32>>>()?;
        vals.sort();
        let total = vals.iter().sum::<u32>() as f32;
        println!("Total: {}ms ; Average: {}ms ; Median: {}ms ; Amplitude: {}ms", total, total / vals.len() as f32, vals[vals.len() / 2], vals[vals.len() - 1] - vals[0]);
        Ok(())
    }
}
