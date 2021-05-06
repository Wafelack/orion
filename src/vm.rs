use crate::{Result, error, OrionError, parser::Literal, bytecode::{Bytecode, Chunk, OpCode}};

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    Single(f32),
    String(String),
    Unit,
    Lambda(u16, Vec<Value>),
}

pub struct VM<const STACK_SIZE: usize> {
    pub input: Bytecode,
    pub stack: Vec<Value>,
}

fn to_val(lit: &Literal) -> Value {
    match lit {
        Literal::Integer(i) => Value::Integer(*i),
        Literal::Single(s) => Value::Single(*s),
        Literal::String(s) => Value::String(s.to_string()),
        Literal::Unit => Value::Unit
    }
}

impl<const STACK_SIZE: usize> VM<STACK_SIZE> {
    pub fn new(input: Bytecode) -> Self {
        Self {
            input,
            stack: Vec::with_capacity(STACK_SIZE),
        }
    }
    fn eval_opcode(&mut self, opcode: OpCode, ctx: &mut Vec<Value>) -> Result<()> {
        match opcode {
            OpCode::LoadConst(id) => self.stack.push(to_val(&self.input.constants[id as usize])),
            OpCode::LoadSym(id) => self.stack.push(ctx[id as usize].clone()),
            OpCode::Def(sym_id) => {
                assert_eq!(ctx.len() as u16, sym_id);
                ctx.push(self.stack.pop().unwrap());
            }
            OpCode::Lambda(chunk_id) => self.stack.push(Value::Lambda(chunk_id, ctx.clone())),
            OpCode::Call(argc) => {
                let mut args = vec![];
                for _ in 0..argc {
                    args.push(self.stack.pop().unwrap());
                }
                let func = self.stack.pop().unwrap();
                if let Value::Lambda(chunk, mut ctx) = func {
                    let chunk = &self.input.chunks[chunk as usize];
                    if chunk.symbols.len() != args.len() {
                        return error!("Expected {} arguments, found {}.", chunk.symbols.len(), args.len());
                    }

                    for idx in 0..chunk.symbols.len() {
                        ctx.push(args[idx].clone());
                    }

                    for instr in chunk.instructions.clone() {
                        self.eval_opcode(instr, &mut ctx)?;
                    }
                } else {
                    return error!("Expected a Lambda, found a {:?}.", func);
                }
            }
            OpCode::Add => {
                let lhs = self.stack.pop().unwrap();
                let rhs = self.stack.pop().unwrap();

                match lhs {
                    Value::Integer(lhs) => match rhs {
                        Value::Integer(rhs) => self.stack.push(Value::Integer(lhs + rhs)),
                        _ => return error!("Expected an Integer, found a {:?}.", rhs),
                    }
                    Value::Single(lhs) => match rhs {
                        Value::Single(rhs) => self.stack.push(Value::Single(lhs + rhs)),
                        _ => return error!("Expected a Single, found a {:?}.", rhs),

                    }
                    _ => return error!("Expected a Single or an Integer, found a {:?}.", lhs),
                }
            }
            _ => todo!(),
        }

        Ok(())
    }
    pub fn eval(&mut self) -> Result<Vec<Value>> {
        let mut ctx= vec![];
        for instr in self.input.instructions.clone() {
            self.eval_opcode(instr, &mut ctx)?;
        }
        Ok(ctx)
    }
}
