use crate::parser::{Expr, Literal};

pub mod bytecode;
use bytecode::{Instruction, Bytecode};

pub mod compiler;

pub struct Compiler {
    input: Vec<Expr>,
    output: Bytecode,
}
