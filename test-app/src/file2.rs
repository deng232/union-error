use crate::error::Result;
use crate::file1;

pub fn read_and_parse(path: &str) -> Result<u32> {
    let text = std::fs::read_to_string(path)?;
    let value = file1::parse_number(text.trim())?;
    Ok(value)
}
