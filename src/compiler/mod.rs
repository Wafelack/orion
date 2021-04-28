use crate::parser::{Expr, Literal};

pub mod bytecode;
use bytecode::Instruction;

pub mod compiler;

pub struct Compiler {
    input: Vec<Expr>,
    output: Vec<Instruction>,
    constants: Vec<Literal>,
}
