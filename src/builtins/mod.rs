pub mod arithmetic;

use std::ops::RangeInclusive;

pub enum ArgsLength {
    OrMore(usize),
    Range(RangeInclusive<usize>),
    Fixed(usize),    
}

impl ArgsLength {
    pub fn display(&self) -> String {
        match self {
            Self::OrMore(u) => format!("{} or more", u),
            Self::Range(r) => {
                let mut toret = String::new();

                for el in r.into_iter() {
                    if el == *r.start() {
                        toret.push_str(&format!("{}", el));
                    } else if el == *r.end() {
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
            Self::Range(r) => r.contains(&length),
            Self::Fixed(u) => *u == length,
        }
    }
}
