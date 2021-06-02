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
use crate::parser::Literal;
use std::time::{SystemTime, UNIX_EPOCH};

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
                let mut to_ret = vec![8];
                to_ret.extend(&file.to_be_bytes());
                to_ret.extend(&line.to_be_bytes());
                to_ret
            }
        }
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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
    pub fn serialize(&self) -> Vec<u8> {
        let mut to_ret = "orion".chars().into_iter().map(|c| c as u8).collect::<Vec<u8>>(); // Magic value
        to_ret.extend_from_slice(&(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32).to_be_bytes()); // Timestamp
        
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
        to_ret.extend(&(self.constructors.len() as u16 * 2 /* Id in sym table */).to_be_bytes());
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
            to_ret.extend(&(serialized.clone().count() as u16).to_be_bytes());
            to_ret.extend(serialized)
        });

        // Instructions
        let serialized = self.instructions.iter().map(|instr| {
            instr.serialize()
        }).flatten();
        to_ret.extend(&(serialized.clone().count() as u16).to_be_bytes());
        to_ret.extend(serialized);

        // Types
        to_ret.extend(&(self.types.len() as u16).to_be_bytes());
        self.types.iter().for_each(|(name, start, end)| {
            to_ret.extend(&start.to_be_bytes());
            to_ret.extend(&end.to_be_bytes());
            to_ret.extend(name.chars().map(|c| c as u8));
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
