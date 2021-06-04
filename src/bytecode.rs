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
use crate::{parser::Literal, error, Result};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum OpCode {
    LoadConst(u16),        // (const_id)
    LoadSym(u16),          // (sym_id)
    Call(u16),             // (argc)
    Builtin(u8, u8),       // (builtin_id, argc)
    Def(u16, u16),         // (sym_id, instructions_length)
    Lambda(u16),           // (chunk_id)
    Constructor(u16, u16), // (constr_idx, valc)
    Tuple(u16, u16),       // (instr_amount, amount)
    Match(u16),            // (match_idx)
    Panic(u16, u16),       // (file_sym, line_sym)
}
impl OpCode {
    pub fn deserialize(ptr: &mut usize, bytes: &[u8]) -> Result<Self> {
        *ptr += 1;
        match bytes[*ptr - 1] {
            0 => Ok(Self::LoadConst(len(ptr, bytes)?)),
            1 => Ok(Self::LoadSym(len(ptr, bytes)?)),
            2 => Ok(Self::Call(len(ptr, bytes)?)),
            3 => {
                *ptr += 2;
                Ok(Self::Builtin(bytes[*ptr - 2], bytes[*ptr - 1])) 
            }
            4 => Ok(Self::Def(len(ptr, bytes)?, len(ptr, bytes)?)),
            5 => Ok(Self::Lambda(len(ptr, bytes)?)),
            6 => Ok(Self::Constructor(len(ptr, bytes)?, len(ptr, bytes)?)),
            7 => Ok(Self::Tuple(len(ptr, bytes)?, len(ptr, bytes)?)),
            8 => Ok(Self::Match(len(ptr, bytes)?)), 
            9 => Ok(Self::Panic(len(ptr, bytes)?, len(ptr, bytes)?)),
            x => error!(=> "Unrecognised op code: {}.", x),
        }
    }
    pub fn serialize(&self) -> Vec<u8> {
        match self {
            Self::LoadConst(id) => {
                let mut to_ret = vec![0];
                to_ret.extend(&id.to_be_bytes());
                to_ret
            }
            Self::LoadSym(id) => {
                let mut to_ret = vec![1];
                to_ret.extend(&id.to_be_bytes());
                to_ret
            }
            Self::Call(argc) => {
                let mut to_ret = vec![2];
                to_ret.extend(&argc.to_be_bytes());
                to_ret
            }
            Self::Builtin(idx, argc) => vec![3, *idx, *argc],
            Self::Def(id, len) => {
                let mut to_ret = vec![4];
                to_ret.extend(&id.to_be_bytes());
                to_ret.extend(&len.to_be_bytes());
                to_ret
            }
            Self::Lambda(id) => {
                let mut to_ret = vec![5];
                to_ret.extend(&id.to_be_bytes());
                to_ret
            }
            Self::Constructor(idx, contained) => {
                let mut to_ret = vec![6];
                to_ret.extend(&idx.to_be_bytes());
                to_ret.extend(&contained.to_be_bytes());
                to_ret
            }
            Self::Tuple(amount, vals) => {
                let mut to_ret = vec![7];
                to_ret.extend(&amount.to_be_bytes());
                to_ret.extend(&vals.to_be_bytes());
                to_ret
            }
            Self::Match(idx) => {
                let mut to_ret = vec![8];
                to_ret.extend(&idx.to_be_bytes());
                to_ret
            }
            Self::Panic(file, line) => {
                let mut to_ret = vec![9];
                to_ret.extend(&file.to_be_bytes());
                to_ret.extend(&line.to_be_bytes());
                to_ret
            }
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Chunk {
    pub instructions: Vec<OpCode>,
    pub reference: Vec<u16>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum BytecodePattern {
    Var(u16), // (sym_idx)
    Constr(u16, Vec<u16>), // (constr_id, [pat_idx])
    Tuple(Vec<u16>), // ([pat_idx])
    Literal(u16), // (const_id)
    Any, // `_` variable 
}

#[derive(PartialEq, Clone, Debug)]
pub struct Bytecode {
    pub types: Vec<(String, u16, u16)>,
    pub chunks: Vec<Chunk>,
    pub matches: Vec<Vec<(u16, Vec<OpCode>)>>,
    pub symbols: Vec<String>,
    pub constants: Vec<Literal>,
    pub instructions: Vec<OpCode>,
    pub patterns: Vec<BytecodePattern>,
    pub constructors: Vec<(u8, u16)>,
}

impl Bytecode {
    pub fn new() -> Self {
        Self {
            types: vec![],
            chunks: vec![],
            symbols: vec![],
            constants: vec![],
            instructions: vec![],
            constructors: vec![],
            matches: vec![],
            patterns: vec![],
        }
    }
    // All numbers here are big endian
    pub fn deserialize(bytes: &[u8]) -> Result<Self> {
        if &bytes[0..5] != "orion".chars().into_iter().map(|c| c as u8).collect::<Vec<u8>>() {
            error!(=> "Invalid bytecode.")
        } else {
            let mut ptr = 5; // Skip timestamp
            let sym_length = len(&mut ptr, bytes)?;
            println!("Sym length.");
            let mut symbols = (0..sym_length).map(|_| string(&mut ptr, bytes)).collect::<Result<Vec<String>>>()?;
            println!("Syms.");
            let consts_length = len(&mut ptr, bytes)?;
            println!("Consts length.");
            let constants = (0..consts_length).map(|_| {
                ptr += 1;
                if bytes[ptr - 1] == 0 {
                    Ok(Literal::String(string(&mut ptr, bytes)?))
                } else if bytes[ptr - 1] == 1 {
                    Ok(Literal::Integer(int(&mut ptr, bytes)?))
                } else if bytes[ptr - 1] == 2 {
                    Ok(Literal::Single(single(&mut ptr, bytes)?))
                } else {
                    error!(=> "Invalid type identifier, expected 0, 1 or 2, found {}.", bytes[ptr - 1])
                }
            }).collect::<Result<Vec<Literal>>>()?;
            println!("Consts.");
            let contrs_length = len(&mut ptr, bytes)?;
            println!("Constrs_length.");
            let constructors = (0..contrs_length).map(|_| {
                let argc = bytes[ptr];
                ptr += 3;
                let idx = (bytes[ptr - 2] as u16) << 8 | (bytes[ptr - 1] as u16);
                (argc, idx)
            }).collect::<Vec<(u8, u16)>>();
            println!("Constrs.");

            let chunks_length = len(&mut ptr, bytes)?;
            println!("Chunks length.");
            let chunks = (0..chunks_length).map(|_| {
                let ref_len = len(&mut ptr, bytes)?;
                let reference = (0..ref_len).map(|_| {
                    len(&mut ptr, bytes)
                }).collect::<Result<Vec<u16>>>()?;
                let instr_len = len(&mut ptr, bytes)? as usize;
                let instructions = (0..instr_len).map(|_| {
                    OpCode::deserialize(&mut ptr, bytes)
                }).collect::<Result<Vec<OpCode>>>()?;
                Ok(Chunk {
                    instructions,
                    reference
                })
            }).collect::<Result<Vec<Chunk>>>()?;
            println!("Chunks.");

            let instrs_length = len(&mut ptr, bytes)?;
            println!("Instrs length.");
            let instructions = (0..instrs_length).map(|_| {
                OpCode::deserialize(&mut ptr, bytes)
            }).collect::<Result<Vec<OpCode>>>()?;
            println!("Instrs.");

            let types_length = len(&mut ptr, bytes)?;
            println!("Types_length.");
            let types = (0..types_length).map(|_| {
                let start = len(&mut ptr, bytes)?;
                let end = len(&mut ptr, bytes)?;
                let t= string(&mut ptr, bytes)?;
                Ok((t, start, end))
            }).collect::<Result<Vec<_>>>()?;
            println!("Types.");

            let patterns_length = len(&mut ptr, bytes)?;
            println!("Patterns_length.");
            let patterns = (0..patterns_length).map(|_| {
                ptr += 1;
                match bytes[ptr - 1] {
                    0 => Ok(BytecodePattern::Var(len(&mut ptr, bytes)?)),
                    1 => {
                        let id = len(&mut ptr, bytes)?;
                        let length = len(&mut ptr, bytes)?;
                        let pats = (0..length).map(|_| len(&mut ptr, bytes)).collect::<Result<Vec<u16>>>()?;
                        Ok(BytecodePattern::Constr(id, pats))
                    }
                    2 => {
                        let length = len(&mut ptr, bytes)?;
                        let pats = (0..length).map(|_| len(&mut ptr, bytes)).collect::<Result<Vec<u16>>>()?;
                        Ok(BytecodePattern::Tuple(pats))
                    }
                    3 => Ok(BytecodePattern::Literal(len(&mut ptr, bytes)?)),
                    4 => Ok(BytecodePattern::Any),
                    _ => error!(=> "Invalid pattern."),
                }
            }).collect::<Result<Vec<BytecodePattern>>>()?;
            println!("Patterns.");
            let matches_length = len(&mut ptr, bytes)?;
            println!("Matches length.");
            let matches = (0..matches_length).map(|_| {
                let match_length = len(&mut ptr, bytes)?;
                Ok((0..match_length).map(|_| {
                    let idx = len(&mut ptr, bytes)?;
                    let instrs_len = len(&mut ptr, bytes)?;
                    let instrs = (0..instrs_len).map(|_| {
                        OpCode::deserialize(&mut ptr, bytes)
                    }).collect::<Result<Vec<OpCode>>>()?;
                    Ok((idx, instrs))
                }).collect::<Result<Vec<(u16, Vec<OpCode>)>>>()?)
            }).collect::<Result<Vec<Vec<(u16, Vec<OpCode>)>>>>()?;
            println!("Matches.");

            Ok(Bytecode {
                types,
                chunks,
                matches,
                symbols,
                constants,
                instructions,
                patterns,
                constructors
            })
        }
    }
    pub fn serialize(&self) -> Vec<u8> {
        let mut to_ret = "orion".chars().into_iter().map(|c| c as u8).collect::<Vec<u8>>(); // Magic value

        // Symbols
        to_ret.extend(&(self.symbols.len() as u16).to_be_bytes()); // Length
        self.symbols.iter().for_each(|sym| {
            sym.chars().for_each(|c| to_ret.push(c as u8));
            to_ret.push(0); // Mark termination
        });

        // Consts
        to_ret.extend(&(self.constants.len() as u16).to_be_bytes()); // Length
        self.constants.iter().for_each(|c| {
            to_ret.push(match c {
                Literal::String(_) => 0,
                Literal::Integer(_) => 1,
                Literal::Single(_) => 2,
            });

            to_ret.extend(match c {
                Literal::Integer(i) => i.to_be_bytes().to_vec(),
                Literal::Single(f) => f.to_bits().to_be_bytes().to_vec(),
                Literal::String(s) => {
                    let mut to_ex = s.chars().map(|c| c as u8).collect::<Vec<_>>();
                    to_ex.push(0); // Mark termination
                    to_ex
                }
            })
        });

        // Constructors
        to_ret.extend(&(self.constructors.len() as u16).to_be_bytes());
        let (mut argc, mut idx) = (vec![], vec![]);
        for (a, i) in &self.constructors {
            argc.push(*a);
            idx.push(*i);
        }
        to_ret.extend(argc);
        to_ret.extend(idx.into_iter().map(|u| u.to_be_bytes().to_vec()).collect::<Vec<Vec<_>>>().into_iter().flatten());

        // Chunks
        to_ret.extend(&(self.chunks.len() as u16).to_be_bytes());
        self.chunks.iter().for_each(|chunk| {
            to_ret.extend(&(chunk.reference.len() as u16).to_be_bytes());
            chunk.reference.iter().for_each(|link| {
                to_ret.extend(&link.to_be_bytes());
            });

            let serialized = chunk.instructions.iter().map(|instr| {
                instr.serialize()
            }).flatten();
            to_ret.extend(&(chunk.instructions.len() as u16).to_be_bytes());
            to_ret.extend(serialized)
        });

        // Instructions
        let serialized = self.instructions.iter().map(|instr| {
            instr.serialize()
        }).flatten();
        to_ret.extend(&(self.instructions.len() as u16).to_be_bytes());
        to_ret.extend(serialized);

        // Types
        to_ret.extend(&(self.types.len() as u16).to_be_bytes());
        self.types.iter().for_each(|(name, start, end)| {
            to_ret.extend(&start.to_be_bytes());
            to_ret.extend(&end.to_be_bytes());
            to_ret.extend(name.chars().map(|c| c as u8));
            to_ret.push(0);
        });

        // Patterns
        to_ret.extend(&(self.patterns.len() as u16).to_be_bytes());
        to_ret.extend(self.patterns.iter().map(|p| {
            match p {
                BytecodePattern::Var(idx) => {
                    let mut to_ret = vec![0];
                    to_ret.extend(&idx.to_be_bytes());
                    to_ret
                }
                BytecodePattern::Constr(id, pats) => {
                    let mut to_ret = vec![1];
                    to_ret.extend(&id.to_be_bytes());
                    to_ret.extend(&(pats.len() as u16).to_be_bytes());
                    to_ret.extend(pats.into_iter().map(|p| {
                        p.to_be_bytes().to_vec()
                    }).flatten());
                    to_ret
                }
                BytecodePattern::Tuple(pats) =>  {
                    let mut to_ret = vec![2];
                    to_ret.extend(&(pats.len() as u16).to_be_bytes());
                    to_ret.extend(pats.into_iter().map(|p| {
                        p.to_be_bytes().to_vec()
                    }).flatten());
                    to_ret
                }
                BytecodePattern::Literal(idx) => {
                    let mut to_ret = vec![3];
                    to_ret.extend(&idx.to_be_bytes());
                    to_ret
                }
                BytecodePattern::Any => vec![4],
            }
        }).flatten());

        // Matches
        to_ret.extend(&(self.matches.len() as u16).to_be_bytes());
        to_ret.extend(self.matches.iter().map(|patterns| {
            let mut to_ret = (patterns.len() as u16).to_be_bytes().to_vec();
            to_ret.extend(patterns.into_iter().map(|(idx, instrs)| {
                let mut to_ret = idx.to_be_bytes().to_vec();
                to_ret.extend(&(instrs.len() as u16).to_be_bytes());
                to_ret.extend(instrs.into_iter().map(|instr| instr.serialize()).flatten());
                to_ret
            }).flatten());
            to_ret
        }).flatten());
        to_ret
    }
}

fn string(ptr: &mut usize, bytes: &[u8]) -> Result<String> {
    let mut to_ret = String::new();
    while *ptr < bytes.len() && bytes[*ptr] != 0 {
        to_ret.push(bytes[*ptr] as char);
        *ptr += 1;
    }
    if bytes.iter().nth(*ptr) == Some(&0) {
        *ptr += 1;
        Ok(to_ret)
    } else {
        error!(=> "Unterminated string.")
    }
}
fn single(ptr: &mut usize, bytes: &[u8]) -> Result<f32> {
    if *ptr + 4 < bytes.len() {
        *ptr += 4;
        Ok(f32::from_bits((bytes[*ptr - 4] as u32) << 24 | (bytes[*ptr - 3] as u32) << 16 | (bytes[*ptr - 2] as u32) << 8 | (bytes[*ptr - 1] as u32)))
    } else {
        error!(=> "Unterminated single precision floating point number.")
    }
}
fn int(ptr: &mut usize, bytes: &[u8]) -> Result<i32> {
    if *ptr + 4 < bytes.len() {
        *ptr += 4;
        Ok(((bytes[*ptr - 4] as u32) << 24 | (bytes[*ptr - 3] as u32) << 16 | (bytes[*ptr - 2] as u32) << 8 | (bytes[*ptr - 1] as u32)) as i32)
    } else {
        error!(=> "Unterminated 32 bits signed integer.")
    }
}
fn len(ptr: &mut usize, bytes: &[u8]) -> Result<u16> {
    if *ptr + 2 < bytes.len() {
        *ptr += 2;
        Ok((bytes[*ptr - 2] as u16) << 8 | (bytes[*ptr - 1] as u16))
    } else {
        error!(=> "Unterminated 16 bits unsigned integer.")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser, compiler::Compiler};

    #[test]
    fn serde() -> Result<()> {
        let tokens = Lexer::new("(def a 42)(def 'impure b 34)", 0).proc_tokens()?;
        let ast = Parser::new(tokens, "TEST").parse()?;
        let (bcode, _, _) = Compiler::new(ast, "TEST", Bytecode::new(), vec![], true, "".to_string(), false)?.compile(vec![])?;
        let ser = bcode.serialize();
        let de = Bytecode::deserialize(&ser)?;
        assert_eq!(bcode, de);
        Ok(())
    }
}
