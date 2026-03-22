use crate::error::Result;
use union_error::located_error;

#[located_error]
#[derive(Debug)]
pub enum LocalErrors {
    Io(std::io::Error),
}

pub fn read_and_parse(path: &str) -> Result<u32> {
    let text = std::fs::read_to_string(path)?;
    let value = crate::file1::parse_number(text.trim())?;
    Ok(value)
}
