use std::fmt::Formatter;

#[derive(Debug)]
pub struct OrionError {
    pub message: String,
    pub line: u32,
    pub file: String,
}

impl std::fmt::Display for OrionError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Error: {}, {}:{}", self.message, self.file, self.line)?;
        Ok(())
    }
}

pub type Result<T> = std::result::Result<T, OrionError>;
#[macro_export]
macro_rules! error {
    () => {
        OrionError {
            message: "".to_owned(),
            line: line!(),
            file: file!().to_owned(),
        }
    };
    ($($msg:tt),*) => {
        {
            use crate::lib::lib::OrionError;
            let mut message = String::new();
            $(
                message.push_str(&format!("{} ", $msg));
            )*
            message.pop();
            OrionError {
                message,
                line: line!(),
                file: file!().to_owned(),
            }
        }
    }
}

