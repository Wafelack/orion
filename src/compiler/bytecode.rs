use crate::parser::Literal;

#[derive(Clone, Debug)]
pub enum Instruction {
    LoadConstant(u16),
    Def(u16),
    LoadVar(u16),
}

#[derive(Clone)]
pub struct Bytecode {
    pub constants: Vec<Literal>,
    pub variables: Vec<String>,
    pub instructions: Vec<Instruction>,
}

impl Bytecode {
    pub fn new() -> Self {
        Self{
            constants: vec![],
            variables: vec![],
            instructions: vec![],
        }
    }
}
