#![forbid(unsafe_code)]

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::panic::Location;

pub use error_union_derive::ErrorUnion;

#[derive(Debug)]
pub struct Located<E> {
    source: E,
    location: &'static Location<'static>,
}

impl<E> Located<E> {
    #[track_caller]
    pub fn new(source: E) -> Self {
        Self {
            source,
            location: Location::caller(),
        }
    }

    pub fn source_ref(&self) -> &E {
        &self.source
    }

    pub fn into_source(self) -> E {
        self.source
    }

    pub fn location(&self) -> &'static Location<'static> {
        self.location
    }
}

impl<E> Display for Located<E>
where
    E: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (at {}:{}:{})",
            self.source,
            self.location.file(),
            self.location.line(),
            self.location.column()
        )
    }
}

impl<E> Error for Located<E>
where
    E: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, ErrorUnion)]
    enum DemoError {
        Parse(Located<std::num::ParseIntError>),
    }

    fn parse_fail() -> Result<u32, DemoError> {
        let n = "x".parse::<u32>()?;
        Ok(n)
    }

    #[test]
    fn located_records_call_site() {
        let err = parse_fail().unwrap_err();
        let display = err.to_string();
        assert!(display.contains("invalid digit found in string"));
        assert!(display.contains("src/lib.rs") || display.contains("tests"));
    }
}
