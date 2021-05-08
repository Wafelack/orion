use crate::{lexer::Lexer, Result, error, OrionError, parser::{Parser, Literal, Expr}, bytecode::{Chunk, Bytecode, OpCode}};
use std::{env, fs, path::Path};
pub struct Compiler {
    input: Vec<Expr>,
    output: Bytecode,
    load_history: Vec<String>,
}

impl Compiler {
    pub fn new(input: Vec<Expr>) -> Self {
        Self {
            input,
            output: Bytecode::new(),
            load_history: vec![],
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
    fn compile_expr(&mut self, expr: Expr, symbols: Option<Vec<String>>) -> Result<Vec<OpCode>> {

        match expr {
            Expr::Literal(lit) => {
                Ok(vec![OpCode::LoadConst(self.register_constant(lit)?)])
            }
            Expr::Var(name) => {
                if !self.output.symbols.contains(&name) {
                    return error!("Variable not in scope: {}.", name);
                }

                Ok(vec![OpCode::LoadSym(symbols.unwrap_or(self.output.symbols.clone()).iter().position(|sym| *sym == name).unwrap() as u16)])
            }
            Expr::Load(files) => {
                let lib_link = match env::var("ORION_LIB") {
                    Ok(v) => v,
                    Err(_) => return error!("ORION_LIB variable does not exist.")
                };

                let mut to_ret = vec![];

                files.into_iter().map(|file| {
                    let lib_path = format!("{}/{}", lib_link, file);
                    let (content, fname) = if Path::new(&lib_path).exists() {
                        match fs::read_to_string(&lib_path) {
                            Ok(c) => (c, lib_path),
                            _ => return error!("Failed to read file: {}.", lib_path),
                        }
                    } else if Path::new(&file).exists() {
                        match fs::read_to_string(&file) {
                            Ok(c) => (c, file.to_string()),
                            _ => return error!("Failed to read file: {}.", file)
                        }
                    } else {
                        return error!("File not found: {}.", file);
                    };

                    if !self.load_history.contains(&fname) {
                        self.load_history.push(fname);
                    } else {
                        return Ok(());
                    }

                    let tokens = Lexer::new(content).proc_tokens()?;
                    let expressions = Parser::new(tokens).parse()?;

                    to_ret.extend(expressions.into_iter().map(|e| {
                        self.compile_expr(e, symbols.clone())
                    }).collect::<Result<Vec<Vec<OpCode>>>>()?.into_iter().flatten().collect::<Vec<OpCode>>());

                    Ok(())
                }).collect::<Result<()>>()?;

                Ok(to_ret)
            }
            Expr::Lambda(mut args, body) => {
                let chunk_syms = args.iter().map(|a| {
                    self.declare(a)
                }).collect::<Result<Vec<_>>>()?;
                args.extend(symbols.unwrap_or(self.output.symbols.clone()));
                let chunk_instrs = self.compile_expr(*body, Some(args))?;
                let chunk = Chunk {
                    instructions: chunk_instrs,
                    symbols: chunk_syms,
                };
                self.output.chunks.push(chunk);
                Ok(vec![OpCode::Lambda(self.output.chunks.len() as u16 - 1)])
            }
            Expr::Def(name, expr) => {
                let idx = self.declare(name)?;
                let mut to_ret = self.compile_expr(*expr, symbols)?;
                to_ret.push(OpCode::Def(idx));
                Ok(to_ret)
            }
            Expr::Call(func, args) => {
                let mut to_ret = self.compile_expr(*func, symbols.clone())?;
                let argc = args.len();
                args.into_iter().map(|a| self.compile_expr(a, symbols.clone())).collect::<Result<Vec<Vec<OpCode>>>>()?.into_iter().for_each(|part| to_ret.extend(part));
                to_ret.push(OpCode::Call(argc as u16));
                Ok(to_ret)
            }
            Expr::Builtin(builtin, args) => {
                match builtin.as_str() {
                    "+" => if args.len() == 2 {
                        self.add(args[0].clone(), args[1].clone(), symbols)
                    } else {
                        return error!("Intrinsic `+` takes 2 arguments, but {} arguments were supplied.", args.len());
                    }
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }        

    }
    fn add(&mut self, lhs: Expr, rhs: Expr, symbols: Option<Vec<String>>) -> Result<Vec<OpCode>> {
        let mut toret = self.compile_expr(lhs, symbols.clone())?;
        toret.extend(self.compile_expr(rhs, symbols)?);
        toret.push(OpCode::Add);
        Ok(toret)
    }
    pub fn compile(&mut self) -> Result<Bytecode> {
        for expr in self.input.clone() {
            let to_push = self.compile_expr(expr, None)?;
            self.output.instructions.extend(to_push);
        }

        Ok(self.output.clone())
    }
}
