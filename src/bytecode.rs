use crate::parser::Literal;

#[derive(Copy, Clone, Debug)]
pub enum OpCode {
    LoadConst(u16),
    LoadSym(u16),
    Call(u16),
    Builtin(u8, u8),
    Def(u16),
    Lambda(u16),
}

#[derive(Clone, Debug)]
pub struct Chunk {
    pub instructions: Vec<OpCode>,
    pub symbols: Vec<u16>,
}

#[derive(Clone, Debug)]
pub struct Bytecode {
    pub chunks: Vec<Chunk>,
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
