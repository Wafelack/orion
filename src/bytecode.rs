use crate::parser::Literal;

#[derive(Copy, Clone, Debug)]
pub enum OpCode {
    LoadConst(u16),
    LoadSym(u16),
    Call(u16),
Builtin(u8, u8),
    Def(u16),
    Add,
}

#[derive(Clone, Debug)]
pub struct Bytecode {
   pub chunks: Vec<Vec<OpCode>>,
   pub symbols: Vec<String>,
   pub constants: Vec<Literal>,
   pub instructions: Vec<OpCode>,
}

impl Bytecode {
    pub fn new() -> Self {
        Self {
            chunks: vec![],
            symbols: vec![],
            constants: vec![],
            instructions: vec![],
        }
    }
}
