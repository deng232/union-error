use crate::error::Result;
use union_error::located_error;

#[located_error]
#[derive(Debug)]
pub enum LocalErrors {
    Parse(std::num::ParseIntError),
}

pub fn parse_number(input: &str) -> Result<u32> {
    let value = input.parse::<u32>()?;
    Ok(value)
}
