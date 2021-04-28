use crate::{Result, parser::Expr, compiler::{Compiler, bytecode::Instruction}};

impl Compiler {
    pub fn new(input: Vec<Expr>) -> Self {
        Self {
            input,
            output: vec![],
            constants: vec![],
        }
    }

    fn compile_expr(&mut self, expr: Expr) -> Result<Vec<Instruction>> {
        match expr {
            Expr::Literal(l) => {
                let mut to_ret = vec![];

                let idx = if !self.constants.contains(&l) {
                    to_ret.push(Instruction::RegisterConstant(l.clone()));
                    self.constants.push(l);
                    self.constants.len() - 1
                } else {
                    self.constants.iter().position(|constant| *constant == l).unwrap()
                };

                to_ret.push(Instruction::LoadConstant(idx as u16));

                Ok(to_ret)
            } 
            _ => todo!(),
        }
    }

    pub fn compile(&mut self) -> Result<Vec<Instruction>> {
        for expr in self.input.clone() {
            let to_push = self.compile_expr(expr)?;
            self.output.extend(to_push);
        }

        Ok(self.output.clone())
    }
}
