/*
 *  Copyright (C) 2021, Wafelack <wafelack@protonmail.com>
 *
 *  ------------------------------------------------------
 *
 *     This file is part of Orion.
 *
 *  Orion is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Orion is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Orion.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::{
    bytecode::{BytecodePattern, Bytecode, Chunk, OpCode},
    error,
    lexer::Lexer,
    parser::{Expr, ExprT, Literal, Parser, Pattern as ParserPattern},
    Result,
};
use std::{fs, path::Path};
pub struct Compiler {
    input: Vec<Expr>,
    output: Bytecode,
    load_history: Vec<String>,
    builtins: Vec<(String, bool)>, // (name, impure?)
    constructors: Vec<String>,
    file: String,
    lib: String,
    repl: bool,
}

impl Compiler {
    pub fn new(input: Vec<Expr>, file: impl ToString, mut bcode: Bytecode, constructors: Vec<String>, already_loaded: bool, lib: String, repl: bool) -> Result<Self> {
        bcode.instructions = vec![];
        let mut new_input = if already_loaded { vec![] } else { vec![Expr::new(ExprT::Load(vec!["prelude.orn".to_string()])).line(0)]};
        new_input.extend(input);
        let mut to_ret = Self {
            input: new_input,
            constructors,
            lib,
            repl,
            output: bcode,
            load_history: vec![],
            builtins: vec![],
            file: file.to_string(),
        };
        to_ret.register_builtin("+", false);
        to_ret.register_builtin("-", false);
        to_ret.register_builtin("*", false);
        to_ret.register_builtin("/", false);
        to_ret.register_builtin("neg", false);
        to_ret.register_builtin("cos", false);
        to_ret.register_builtin("sin", false);
        to_ret.register_builtin("tan", false);
        to_ret.register_builtin("acos", false);
        to_ret.register_builtin("asin", false);
        to_ret.register_builtin("atan", false);

        to_ret.register_builtin("format", false);

        to_ret.register_builtin("putStr", true);
        to_ret.register_builtin("getLine", true);

        to_ret.register_builtin("type", false);
        to_ret.register_builtin("_cmp", false);

        Ok(to_ret)
    }
    fn register_builtin(&mut self, name: impl ToString, impure: bool) {
        self.builtins.push((name.to_string(), impure))
    }
    fn register_constant(&mut self, constant: Literal, line: usize) -> Result<u16> {
        if !self.output.constants.contains(&constant) {
            self.output.constants.push(constant.clone());
        }
        if self.output.constants.len() > u16::MAX as usize {
            error!(self.file, line => "Too much constants are used.")
        } else {
            Ok(self
               .output
               .constants
               .iter()
               .position(|c| c == &constant)
               .unwrap() as u16)
        }
    }
    fn register_constructor(&mut self, name: impl ToString, symbols: Vec<(String, bool)>, contained_amount: u8, line: usize) -> Result<Vec<(String, bool)>> {
        let name = name.to_string();
        if self.constructors.contains(&name) {
            error!(
                self.file,
                line =>
                "Enum Variant {} has already been defined (Index 0x{:04x})",
                &name,
                self.constructors
                .iter()
                .position(|var| var.to_string() == name)
                .unwrap()
                )
        } else {
            self.constructors.push(name.clone());
            let (idx, symbols) = self.declare(name, symbols, false, line)?;
            self.output.constructors.push((contained_amount, idx));
            Ok(symbols)
        }
    }
    fn get_constructor(&self, name: impl ToString, line: usize) -> Result<(u8, u16)> {
        let name = name.to_string();
        if self.constructors.contains(&name) {
            let idx = self
                .constructors
                .iter()
                .position(|variant| name == variant.to_string())
                .unwrap();
            Ok((self.output.constructors[idx].0, idx as u16))
        } else {
            error!(self.file, line => "Enum variant {} does not exist.", name)
        }
    }
    fn declare(
        &mut self,
        name: impl ToString,
        mut symbols: Vec<(String, bool)>,
        impure: bool,
        line: usize,
        ) -> Result<(u16, Vec<(String, bool)>)> {
        if symbols.len() >= u16::MAX as usize {
            error!(self.file, line => "Too much symbols are declared.")
        } else {
            Ok((
                    if symbols.contains(&(name.to_string(), impure))
                    || symbols.contains(&(name.to_string(), !impure))
                    {
                        symbols
                            .iter()
                            .position(|s| s.0 == name.to_string())
                            .unwrap()
                    } else {
                        symbols.push((name.to_string(), impure));
                        symbols.len() - 1
                    } as u16,
                    symbols,
                    ))
        }
    }
    fn load_file(
        &mut self,
        fname: impl ToString,
        mut symbols: Vec<(String, bool)>,
        line: usize,
        ) -> Result<(Vec<OpCode>, Vec<(String, bool)>)> {
        let fname = fname.to_string();
        if self.load_history.contains(&fname) {
            // Avoid error-prone reloading if file has already been loaded.
            Ok((vec![], symbols))
        } else {
            self.load_history.push(fname.clone());
            match fs::read_to_string(&fname) {
                Ok(content) => {
                    let tokens = Lexer::new(content, &fname).proc_tokens()?;
                    let expressions = Parser::new(tokens, fname).parse()?;
                    Ok((
                            expressions
                            .into_iter()
                            .map(|e| {
                                let to_ret = self.compile_expr(e, symbols.clone(), true)?;
                                symbols = to_ret.1; // Update symbols.
                                Ok(to_ret.0)
                            })
                            .collect::<Result<Vec<Vec<OpCode>>>>()?
                            .into_iter()
                            .flatten()
                            .collect(),
                            symbols,
                            ))
                }
                Err(e) => error!(self.file, line => "Failed to read file: {}: {}.", fname, e),
            }
        }
    }
    fn compile_expr(
        &mut self,
        expr: Expr,
        mut symbols: Vec<(String, bool)>,
        impure: bool,
        ) -> Result<(Vec<OpCode>, Vec<(String, bool)>)> {
        match expr.exprt.clone() {
            ExprT::Literal(lit) => Ok((
                    vec![(OpCode::LoadConst(self.register_constant(lit, expr.line)?))],
                    symbols,
                    )),
            ExprT::Var(name) => {
                if name.as_str() == "__LINE__" {
                    self.compile_expr(Expr::new(ExprT::Literal(Literal::Integer(expr.line as i32))).line(expr.line), symbols, impure)
                } else if name.as_str() == "__FILE__" {
                    self.compile_expr(Expr::new(ExprT::Literal(Literal::String(self.file.clone()))).line(expr.line), symbols, impure)
                } else if !symbols.contains(&(name.clone(), impure)) {
                    if impure && symbols.contains(&(name.clone(), false)) {
                        let (idx, symbols) = self.declare(name, symbols, impure, expr.line)?;
                        Ok((vec![OpCode::LoadSym(idx)], symbols))
                    } else if !impure && symbols.contains(&(name.clone(), true)) {
                        error!(
                            self.file,
                            expr.line =>
                            "Impure function used out of an `impure` declaration: {}",
                            name
                            )
                    } else {
                        error!(self.file, expr.line => "Variable not in scope: {}.", name)
                    }
                } else {
                    let (idx, symbols) = self.declare(name, symbols, impure, expr.line)?;
                    Ok((vec![OpCode::LoadSym(idx)], symbols))
                }
            }
            ExprT::Load(files) => {
                let lib_link = self.lib.to_string();

                let instrs = files
                        .into_iter()
                        .map(|file| {
                            let lib_path = format!("{}/{}", lib_link, file);
                            let fname = if Path::new(&lib_path).exists() {
                                Ok(lib_path)
                            } else if Path::new(&file).exists() {
                                Ok(file)
                            } else {
                                error!(self.file, expr.line => "File not found: {}.", file)
                            }?;

                            let to_ret = self.load_file(fname, symbols.clone(), expr.line)?;
                            symbols = to_ret.1; // Update symbols.
                            Ok(to_ret.0)
                        })
                        .collect::<Result<Vec<Vec<OpCode>>>>()?
                        .into_iter()
                        .flatten()
                        .collect::<Vec<OpCode>>();

                Ok((
                        instrs,
                        symbols,
                        ))
            }
            ExprT::Def(name, value, purity) => {
                let (idx, symbols) = self.declare(name, symbols, purity, expr.line)?;
                let (to_push, symbols) = self.compile_expr(*value, symbols, purity)?; // Update symbols.
                let mut to_ret = vec![OpCode::Def(idx, to_push.len() as u16)];
                to_ret.extend(to_push);
                Ok((to_ret, symbols))
            }
            ExprT::Call(func, args) => {
                let (mut to_ret, mut symbols) = self.compile_expr(*func, symbols, impure)?; // The λ to execute.
                let argc = args.len() as u16;
                to_ret.extend(
                    // Push arguments onto the stack, and keep the amount in order to pop all the arguments.
                    args.into_iter()
                    .map(|a| {
                        let (opcodes, syms) = self.compile_expr(a, symbols.clone(), impure)?;
                        symbols = syms; // Update symbols.
                        Ok(opcodes)
                    })
                    .collect::<Result<Vec<Vec<OpCode>>>>()?
                    .into_iter()
                    .flatten()
                    .collect::<Vec<OpCode>>(),
                    );
                to_ret.push(OpCode::Call(argc));
                Ok((to_ret, symbols))
            }
            ExprT::Begin(expressions) => {
                let instructions = expressions.into_iter().map(|expr| {
                    let (instruction, new_syms) = self.compile_expr(expr, symbols.clone(), impure)?;
                    symbols = new_syms;
                    Ok(instruction)
                }).collect::<Result<Vec<Vec<OpCode>>>>()?.into_iter().flatten().collect::<Vec<OpCode>>();
                self.output.chunks.push(Chunk {
                    instructions,
                    reference: vec![],
                });
                Ok((vec![OpCode::Lambda(self.output.chunks.len() as u16 - 1), OpCode::Call(0)], symbols))
            }
            ExprT::Lambda(args, body) => {
                let args_reference = args
                    .iter()
                    .map(|a| {
                        let (idx, syms) = self.declare(a, symbols.clone(), false, expr.line)?;
                        symbols = syms;
                        Ok(idx)
                    })
                .collect::<Result<Vec<_>>>()?; // Position in the symbol table for each arg.
                let run_with = symbols
                    .iter()
                    .enumerate()
                    .map(|(idx, sym)| {
                        match args_reference.iter().position(|id| *id as usize == idx) {
                            // Check if the current symbol is part of the arguments
                            // If it is, then replace it by the argument's value, else
                            // use the value that is already in the table.
                            Some(arg_i) => (args[arg_i].to_string(), false),
                            None => sym.clone(),
                        }
                    })
                .collect::<Vec<(String, bool)>>();
                let (chunk_instructions, symbols) = self.compile_expr(*body, run_with, impure)?;
                self.output.chunks.push(Chunk {
                    instructions: chunk_instructions,
                    reference: args_reference,
                });
                Ok((
                        vec![OpCode::Lambda(self.output.chunks.len() as u16 - 1)],
                        symbols,
                        ))
            }
            ExprT::Builtin(name, args) => {
                let argc = args.len();
                let mut to_ret = args
                    .into_iter()
                    .map(|arg| {
                        let (compiled, new_syms) =
                            self.compile_expr(arg, symbols.clone(), impure)?;
                        symbols = new_syms; // Update symbols.
                        Ok(compiled)
                    })
                .collect::<Result<Vec<Vec<OpCode>>>>()?
                    .into_iter()
                    .flatten()
                    .collect::<Vec<OpCode>>();


                if name.as_str() == "panic" {
                    if argc != 1 {
                        return error!(self.file, expr.line => "Intrisic panic takes 1 argument but {} arguments were supplied.", argc)
                    } else {
                        to_ret.push(OpCode::Panic(self.register_constant(Literal::String(self.file.clone()), expr.line)?, self.register_constant(Literal::Integer(expr.line as i32), expr.line)?));
                        return Ok((to_ret, symbols));
                    }
                }

                let idx = self
                    .builtins
                    .iter()
                    .position(|builtin| builtin.0 == name)
                    .map_or(error!(self.file, expr.line => "No such builtin: {}.", name), |i| Ok(i))?;
                let impure_builtin = self.builtins[idx as usize].1;
                if !impure && impure_builtin {
                    return error!(self.file, expr.line => "Impure builtin used out of an `impure` function: {}.", name);
                }
                to_ret.push(OpCode::Builtin(idx as u8, argc as u8));
                Ok((to_ret, symbols))
            }
            ExprT::Enum(name, constructors) => {
                let start = self.output.constructors.len() as u16;
                constructors
                    .into_iter()
                    .map(|(k, v)| {
                        symbols = self.register_constructor(k, symbols.clone(), v, expr.line)?;
                        Ok(())
                    })
                .collect::<Result<()>>()?;
                let end = self.output.constructors.len() as u16 - 1;
                self.output.types.push((name, start, end));
                Ok((vec![], symbols))
            }
            ExprT::Constr(name, contained) => {
                let (amount, idx) = self.get_constructor(&name, expr.line)?;
                self.check_constr(idx, amount, contained.len() as u8, expr.line)?;
                if amount != contained.len() as u8 {
                    error!(
                        self.file,
                        expr.line =>
                        "Enum Constructor {} takes {} values, but {} values were given.",
                        name,
                        amount,
                        contained.len()
                        )
                } else {
                    let values = contained
                        .into_iter()
                        .map(|expr| {
                            let (compiled, new_syms) =
                                self.compile_expr(expr, symbols.clone(), impure)?;
                            symbols = new_syms;
                            Ok(compiled)
                        })
                    .collect::<Result<Vec<Vec<OpCode>>>>()?
                        .into_iter()
                        .flatten()
                        .collect::<Vec<OpCode>>();
                    let mut to_ret = vec![OpCode::Constructor(idx, values.len() as u16)];
                    to_ret.extend(values);
                    Ok((to_ret, symbols))
                }
            }
            ExprT::Tuple(vals) => {
                let length = vals.len();
                let values = vals
                    .into_iter()
                    .map(|expr| {
                        let (compiled, new_syms) =
                            self.compile_expr(expr, symbols.clone(), impure)?;
                        symbols = new_syms;
                        Ok(compiled)
                    })
                .collect::<Result<Vec<Vec<OpCode>>>>()?
                    .into_iter()
                    .flatten()
                    .collect::<Vec<OpCode>>();
                let op_count = values.len();
                let mut to_ret = vec![OpCode::Tuple(op_count as u16, length as u16)];
                to_ret.extend(values);
                Ok((to_ret, symbols))
            }
            ExprT::Match(expr, patterns) => {
                let (mut compiled, mut symbols) = self.compile_expr(*expr.clone(), symbols, impure)?;
                let match_content = patterns.into_iter().map(|(pat, expr)| {
                    let (pat_id, new_symbols) = self.declare_pat(pat, symbols.clone(), impure, expr.line)?;
                    symbols = new_symbols;
                    let (compiled, new_syms) = self.compile_expr(expr, symbols.clone(), impure)?;
                    symbols = new_syms;
                    Ok((pat_id, compiled))
                }).collect::<Result<Vec<(u16, Vec<OpCode>)>>>()?;

                let idx = if self.output.matches.contains(&match_content) {
                    self.output.matches.iter().position(|m| m == &match_content).unwrap()
                } else {
                    self.output.matches.push(match_content);
                    self.output.matches.len() - 1
                } as u16;
                compiled.push(OpCode::Match(idx));
                Ok((compiled, symbols))
            }
        }
    }
    fn declare_pat(&mut self, pat: ParserPattern, mut symbols: Vec<(String, bool)>, impure: bool, line: usize) -> Result<(u16, Vec<(String, bool)>)> {
        let flattened = match pat {
            ParserPattern::Var(s) => {
                if s.as_str() == "_" {
                    BytecodePattern::Any
                } else {
                    let (sym_id, new_symbols) = self.declare(s, symbols.clone(), impure, line)?;
                    symbols = new_symbols;
                    BytecodePattern::Var(sym_id)
                }
            }
            ParserPattern::Constr(constr, inside) => {
                let (amount, constr_id)= self.get_constructor(constr, line)?;
                self.check_constr(constr_id, amount, inside.len() as u8, line)?;
                BytecodePattern::Constr(constr_id, inside.into_iter().map(|pat| {
                    let (idx, new_syms) = self.declare_pat(pat, symbols.clone(), impure, line)?;
                    symbols = new_syms;
                    Ok(idx)
                }).collect::<Result<Vec<u16>>>()?)
            }
            ParserPattern::Tuple(inside) => {
                BytecodePattern::Tuple(inside.into_iter().map(|pat| {
                    let (idx, new_syms) = self.declare_pat(pat, symbols.clone(), impure, line)?;
                    symbols = new_syms;
                    Ok(idx)
                }).collect::<Result<Vec<u16>>>()?)
            }
            ParserPattern::Literal(lit) => {
                let idx = self.register_constant(lit, line)?;
                BytecodePattern::Literal(idx)
            }
        };

        Ok((if self.output.patterns.contains(&flattened) {
            self.output.patterns.iter().position(|pat| pat == &flattened).unwrap() as u16
        } else {
            self.output.patterns.push(flattened);
            self.output.patterns.len() as u16 - 1
        }, symbols))
    }
    fn check_constr(&self, idx: u16, expected: u8, given: u8, line: usize) -> Result<()> {
        if given != expected {
            error!(self.file, line => "Constructor {} takes {} values, but {} values were given.", self.constructors[idx as usize], expected, given)
        } else {
            Ok(())
        }
    }
    pub fn compile(&mut self, mut symbols: Vec<(String, bool)>) -> Result<(Bytecode, Vec<(String, bool)>, Vec<String>)> {
        for expr in self.input.clone() {
            let (to_push, new_symbols) = self.compile_expr(expr, symbols, self.repl)?;
            symbols = new_symbols;
            self.output.instructions.extend(to_push);
        }
        self.output.symbols = symbols
            .iter()
            .map(|(name, _)| name.to_string())
            .collect::<Vec<String>>();

        if self.output.symbols.contains(&"main".to_string()) {
            self.output.instructions.extend(vec![OpCode::LoadSym(self.output.symbols.iter().position(|s| s == "main").unwrap() as u16), OpCode::Call(0)]);
        }

        Ok((self.output.clone(), symbols, self.constructors.clone()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn def() -> Result<()> {
        let tokens = Lexer::new("(def a 42)(def 'impure b 34)", 0).proc_tokens()?;
        let ast = Parser::new(tokens, "TEST").parse()?;
        let (bcode, symbols, _) = Compiler::new(ast, "TEST", Bytecode::new(), vec![], true, "".to_string(), false)?.compile(vec![])?;
        assert_eq!(bcode.instructions, vec![OpCode::Def(0, 1),OpCode::LoadConst(0),  OpCode::Def(1, 1), OpCode::LoadConst(1)]);
        assert_eq!(symbols, vec![("a".to_string(), false), ("b".to_string(), true)]);
        Ok(())
    }
}
