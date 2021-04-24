pub mod arithmetic;
pub mod io;
pub mod string;
pub mod misc;

pub enum ArgsLength {
    OrMore(usize),
    Fixed(usize),    
}

impl ArgsLength {
    pub fn display(&self) -> String {
        match self {
            Self::OrMore(u) => format!("{} or more", u),
            Self::Fixed(u) => format!("{}", u),

        }
    }
    pub fn contains(&self, length: usize) -> bool {
        match self  {
            Self::OrMore(u) => length >= *u,
            Self::Fixed(u) => *u == length,
        }
    }
}
