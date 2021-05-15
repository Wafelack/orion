use crate::{    
    bytecode::{Bytecode, Chunk, OpCode},
    error,
    parser::Literal,
    OrionError, Result,
};

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    Single(f32),
    String(String),
    Unit,
    Lambda(u16),
    Constructor(u16, Vec<Value>),
    Tuple(Vec<Value>),
    Initialzing,
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
        Literal::Unit => Value::Unit,
    }
}

impl<const STACK_SIZE: usize> VM<STACK_SIZE> {
    pub fn new(input: Bytecode) -> Self {
        let mut to_ret = Self {
            input,
            stack: Vec::with_capacity(STACK_SIZE),
            builtins: vec![],
            ip: 0,
        };
        to_ret.register_builtin(Self::add, 2);
        to_ret.register_builtin(Self::dbg, 1);
        to_ret
    }
    fn dbg(&mut self, _: &mut Vec<Value>) -> Result<Value> {
        println!("{:?}", self.stack.pop().unwrap());
        Ok(Value::Unit)
    }
    fn add(&mut self, _: &mut Vec<Value>) -> Result<Value> {
        let lhs = self.stack.pop().unwrap();
        let rhs = self.stack.pop().unwrap();

        match lhs {
            Value::Integer(lhs) => match rhs {
                Value::Integer(rhs) => Ok(Value::Integer(lhs + rhs)),
                _ => error!("Expected an Integer, found a {:?}.", rhs),
            },
            Value::Single(lhs) => match rhs {
                Value::Single(rhs) => Ok(Value::Single(lhs + rhs)),
                _ => error!("Expected a Single, found a {:?}.", rhs),
            },
            _ => error!("Expected a Single or an Integer, found a {:?}.", lhs),
        }
    }
    fn register_builtin(
        &mut self,
        func: fn(&mut VM<STACK_SIZE>, &mut Vec<Value>) -> Result<Value>,
        argc: u8,
        ) {
        self.builtins.push((func, argc))
    }
    fn eval_opcode(&mut self, opcode: OpCode, ctx: &mut Vec<Value>) -> Result<()> {
        match opcode {
            OpCode::LoadConst(id) => self.stack.push(to_val(&self.input.constants[id as usize])),
            OpCode::LoadSym(id) => {
                println!("Symbols: {:?}", self.input.symbols);
                println!("CTX: {:?}", ctx);
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
                let popped = self.stack.pop().unwrap();
                ctx[sym_id as usize] = popped;
            }
            OpCode::Lambda(chunk_id) => self.stack.push(Value::Lambda(chunk_id)),
            OpCode::Call(argc) => {
                let mut args = vec![];
                for _ in 0..argc {
                    args.push(self.stack.pop().unwrap());
                }
                args.reverse();
                let func = self.stack.pop().unwrap();
                if let Value::Lambda(chunk) = func {
                    let chunk = &self.input.chunks[chunk as usize];
                    if chunk.reference.len() != args.len() {
                        return error!(
                            "Expected {} arguments, found {}.",
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
                    return error!("Expected a Lambda, found a {:?}.", func);
                }
            }
            OpCode::Builtin(idx, argc) => {
                let (f, f_argc) = self.builtins[idx as usize];
                if f_argc != argc {
                    return error!(
                        "Builtin 0x{:02x} takes {} arguments, but {} arguments were supplied.",
                        idx, f_argc, argc
                        );
                }
                let to_push = f(self, ctx)?;
                self.stack.push(to_push);
            }
            OpCode::Constructor(idx, to_eval) => {
                let amount = self.input.constructors[idx as usize];
                for i in 1..=amount {
                    let instruction = self.input.instructions[self.ip + i as usize];
                    self.eval_opcode(instruction, ctx)?;
                }
                self.ip += amount as usize;
                let vals = (0..to_eval)
                    .map(|_| self.stack.pop().unwrap())
                    .collect::<Vec<Value>>();
                self.stack.push(Value::Constructor(idx, vals));
            }
            OpCode::Tuple(amount, to_eval) => {
                for i in 1..=amount {
                    let instruction = self.input.instructions[self.ip + i as usize];
                    self.eval_opcode(instruction, ctx)?;
                }
                self.ip += amount as usize;
                let vals = (0..to_eval)
                    .map(|_| self.stack.pop().unwrap())
                    .collect::<Vec<Value>>();
                self.stack.push(Value::Tuple(vals))
            }
        }

        Ok(())
    }
    pub fn eval(&mut self) -> Result<Vec<Value>> {
        let mut ctx = vec![];
        while self.ip < self.input.instructions.len() {
            let instruction = self.input.instructions[self.ip];
            self.eval_opcode(instruction, &mut ctx)?;
            self.ip += 1;
        }
        Ok(ctx)
    }
}
