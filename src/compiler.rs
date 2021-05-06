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
    fn declare(&mut self, name: impl ToString) -> Result<u16> {
        if self.output.symbols.len() > u16::MAX as usize{
            return error!("Too much symbols are declared.");
        }

        if !self.output.symbols.contains(&name.to_string()) {
            self.output.symbols.push(name.to_string());
        }

        Ok(self.output.symbols.iter().position(|s| s == &name.to_string()).unwrap() as u16)
    }
    fn register_constant(&mut self, constant: Literal) -> Result<u16> {
        if !self.output.constants.contains(&constant) {
            self.output.constants.push(constant.clone());
        }

        if self.output.constants.len() > u16::MAX as usize{
            error!("Too much constants are used.")
        } else {
            Ok(self.output.constants.iter().position(|c| c == &constant).unwrap() as u16)
        }
    }
    fn compile_expr(&mut self, expr: Expr) -> Result<Vec<OpCode>> {

        match expr {
            Expr::Literal(lit) => {
                Ok(vec![OpCode::LoadConst(self.register_constant(lit)?)])
            }
            Expr::Var(name) => {
                if !self.output.symbols.contains(&name) {
                    return error!("Variable not in scope: {}.", name);
                }

                Ok(vec![OpCode::LoadSym(self.output.symbols.iter().position(|sym| *sym == name).unwrap() as u16)])
            }
            Expr::Lambda(args, body) => {

                args.into_iter().map(|a| self.declare(a)).collect::<Result<Vec<_>>>()?;
                let chunk = self.compile_expr(*body)?;
                self.output.chunks.push(chunk);
                Ok(vec![OpCode::Lambda(self.output.chunks.len() as u16 - 1)])
            }
            Expr::Def(name, expr) => {
                if self.output.symbols.contains(&name) {
                    error!("Multiple declarations of `{}`.", name)
                } else {
                    self.declare(name)?;
                    let mut to_ret = self.compile_expr(*expr)?;
                    to_ret.push(OpCode::Def(self.output.symbols.len() as u16 - 1));
                    Ok(to_ret)
                }
            }
            Expr::Call(func, args) => {
                let mut to_ret = self.compile_expr(*func)?;
                let argc = args.len();
                args.into_iter().map(|a| self.compile_expr(a)).collect::<Result<Vec<Vec<OpCode>>>>()?.into_iter().for_each(|part| to_ret.extend(part));
                to_ret.push(OpCode::Call(argc as u16));
                Ok(to_ret)
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
