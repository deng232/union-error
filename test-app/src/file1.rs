use crate::error::Result;

pub fn parse_number(input: &str) -> Result<u32> {
    let value = input.parse::<u32>()?;
    Ok(value)
}
