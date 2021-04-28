use crate::parser::Literal;

#[derive(Clone, Debug)]
pub enum Instruction {
    RegisterConstant(Literal),
    LoadConstant(u16),
}
