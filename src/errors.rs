#[derive(Debug)]
pub struct OrionError(pub String);

pub type Result<T> = std::result::Result<T, OrionError>;

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        Err(OrionError(format_args!($($arg)*).to_string()))
    }
}
