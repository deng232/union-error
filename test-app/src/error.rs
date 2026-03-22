use union_error::{ErrorUnion, Located};

#[derive(Debug, ErrorUnion)]
pub enum AppError {
    Parse(Located<std::num::ParseIntError>),
    Io(Located<std::io::Error>),
}

pub type Result<T> = std::result::Result<T, AppError>;
