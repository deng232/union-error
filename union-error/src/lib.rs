#![forbid(unsafe_code)]

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::panic::Location;

pub use union_error_derive::{error_union, located_error, ErrorUnion};

#[doc(hidden)]
pub mod __private {
    #[derive(Debug, Clone, Copy)]
    pub struct LeafSpec {
        pub variant_name: &'static str,
        pub leaf_type_name: &'static str,
    }

    pub trait LocatedErrorMetadata {
        const LEAVES: &'static [LeafSpec];
    }
}

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
