use crate::{Result, error, OrionError, parser::Expr, compiler::{Compiler, bytecode::{Instruction, Bytecode}}};

impl Compiler {
    pub fn new(input: Vec<Expr>) -> Self {
        Self {
            input,
            output: Bytecode::new(),
        }
    }

    fn compile_expr(&mut self, expr: Expr) -> Result<Vec<Instruction>> {
        match expr {
            Expr::Literal(l) => {
                let mut to_ret = vec![];

                let idx = if !self.output.constants.contains(&l) {
                    self.output.constants.push(l);
                    self.output.constants.len() - 1
                } else {
                    self.output.constants.iter().position(|constant| *constant == l).unwrap()
                };

                to_ret.push(Instruction::LoadConstant(idx as u16));

                Ok(to_ret)
            }
            Expr::Def(name, expr) => {

                if self.output.variables.contains(&name) {
                    return error!("Multiple declarations of `{}`.", name);
                }

                let mut to_ret = self.compile_expr(*expr)?; 
                to_ret.push(Instruction::Def(self.output.variables.len() as u16));
                to_ret.push(Instruction::LoadConstant(0));

                self.output.variables.push(name);

                Ok(to_ret)
            }
            Expr::Var(name) => {
                if !self.output.variables.contains(&name) {
                    error!("Variable not in scope: {}.", name)
                } else {
                    Ok(vec![Instruction::LoadVar(self.output.variables.iter().position(|e| e == &name).unwrap() as u16)])
                }
            }
            Expr::Call(func, arg) => {
                let mut to_ret = self.compile_expr(*func)?;
                to_ret.extend(self.compile_expr(*arg)?);
                to_ret.push(Instruction::Call);
                Ok(to_ret)
            }
            _ => todo!(),
        }
    }

    pub fn compile(&mut self) -> Result<Bytecode> {
        for expr in self.input.clone() {
            let to_push = self.compile_expr(expr)?;
            self.output.instructions.extend(to_push);
        }

        Ok(self.output.clone())
    }
}
