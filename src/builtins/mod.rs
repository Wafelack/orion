pub mod arithmetic;
pub mod io;
pub mod string;

use std::ops::RangeInclusive;

pub enum ArgsLength {
    OrMore(usize),
    Range(usize, usize),
    Fixed(usize),    
}

impl ArgsLength {
    pub fn display(&self) -> String {
        match self {
            Self::OrMore(u) => format!("{} or more", u),
            Self::Range(a, b) => {
                let mut toret = String::new();

                for el in *a..=*b {
                    if el == *a {
                        toret.push_str(&format!("{}", el));
                    } else if el == *b {
                        toret.push_str(&format!(" or {}", el));
                    } else {
                        toret.push_str(&format!(", {}", el));
                    }
                }

                toret

            }
            Self::Fixed(u) => format!("{}", u),

        }
    }
    pub fn contains(&self, length: usize) -> bool {
        match self  {
            Self::OrMore(u) => length >= *u,
            Self::Range(a, b) => (*a..=*b).contains(&length),
            Self::Fixed(u) => *u == length,
        }
    }
}
