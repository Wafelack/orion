use crate::{Result, error, OrionError, parser::{Literal, Expr}, bytecode::{Bytecode, OpCode}};

pub struct Compiler {
    input: Vec<Expr>,
    output: Bytecode,
}

impl Compiler {
    pub fn new(input: Vec<Expr>) -> Self {
        Self {
            input,
            output: Bytecode::new(),
        }
    }
    fn compile_expr(&mut self, expr: Expr) -> Result<Vec<OpCode>> {

        match expr {
            Expr::Literal(lit) => {
                if !self.output.constants.contains(&lit) {
                    self.output.constants.push(lit.clone());
                }

                if self.output.constants.len() > u16::MAX as usize{
                    return error!("Too much constants are used.");
                }

                Ok(vec![OpCode::LoadConst(self.output.constants.iter().position(|c| *c == lit).unwrap() as u16)])
            }
            Expr::Lambda(arg, body) => {

                if self.output.symbols.len() > u16::MAX as usize{
                    return error!("Too much symbols are declared.");
                } else if !self.output.symbols.contains(&arg) {
                    self.output.symbols.push(arg.clone());
                }
                let mut toret = self.compile_expr(*body)?;
                toret.push(OpCode::LoadSym(self.output.symbols.iter().position(|sym| *sym == arg).unwrap_or_else(|| {
                    self.output.symbols.push(arg);
                    self.output.symbols.len()
                }) as u16));

                Ok(toret)

            }
            Expr::Var(name) => {
                if !self.output.symbols.contains(&name) {
                    return error!("Variable not in scope: {}.", name);
                }

                if self.output.symbols.len() > u16::MAX as usize{
                    return error!("Too much symbols are declared.");
                }

                Ok(vec![OpCode::LoadSym(self.output.symbols.iter().position(|sym| *sym == name).unwrap() as u16)])
            }
            Expr::Def(name, expr) => {
                if self.output.symbols.contains(&name) {
                    error!("Multiple declarations of `{}`.", name)
                } else {

                    if self.output.symbols.len() > u16::MAX as usize{
                        return error!("Too much symbols are declared.");
                    }

                    let mut to_ret = self.compile_expr(*expr)?;
                    to_ret.push(OpCode::Def(self.output.symbols.len() as u16));
                    self.output.symbols.push(name);
                    Ok(to_ret)
                }
            }
            Expr::Builtin(builtin, args) => {
                match builtin.as_str() {
                    "+" => if args.len() == 2 {
                        self.add(args[0].clone(), args[1].clone())
                    } else {
                        return error!("Intrinsic `+` takes 2 arguments, but {} arguments were supplied.", args.len());
                    }
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }        

    }
    fn add(&mut self, lhs: Expr, rhs: Expr) -> Result<Vec<OpCode>> {
        let mut toret = self.compile_expr(lhs)?;
        toret.extend(self.compile_expr(rhs)?);
        toret.push(OpCode::Add);
        Ok(toret)
    }
    pub fn compile(&mut self) -> Result<Bytecode> {
        for expr in self.input.clone() {
            let to_push = self.compile_expr(expr)?;
            self.output.instructions.extend(to_push);
        }

        Ok(self.output.clone())
    }
}
