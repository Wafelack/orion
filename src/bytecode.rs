use crate::parser::{Literal, self};

#[derive(Copy, Clone, Debug)]
pub enum OpCode {
    LoadConst(u16),        // (const_id)
    LoadSym(u16),          // (sym_id)
    Call(u16),             // (argc)
    Builtin(u8, u8),       // (builtin_id, argc)
    Def(u16, u16),         // (sym_id, instructions_length)
    Lambda(u16),           // (chunk_id)
    Constructor(u16, u16), // (constr_idx, to_eval)
    Tuple(u16),       // (amount, to_eval)
}

#[derive(Clone, Debug)]
pub struct Chunk {
    pub instructions: Vec<OpCode>,
    pub reference: Vec<u16>,
}

#[derive(Clone, Debug)]
pub struct Pattern {
    pub pat: parser::Pattern,
    pub to_exec: Vec<OpCode>,
}
#[derive(Clone, Debug)]
pub struct Match {
    pub expression: Vec<OpCode>,
    pub patterns: Vec<Pattern>,
}
#[derive(Clone, Debug)]
pub struct Bytecode {
    pub chunks: Vec<Chunk>,
    pub matches: Vec<Match>,
    pub symbols: Vec<String>,
    pub constants: Vec<Literal>,
    pub instructions: Vec<OpCode>,
    pub constructors: Vec<u8>,
}

impl Bytecode {
    pub fn new() -> Self {
        Self {
            chunks: vec![],
            symbols: vec![],
            constants: vec![],
            instructions: vec![],
            constructors: vec![],
            matches: vec![],
        }
    }
}
