use std::error::Error;
use std::fmt::{Display, Formatter};

/// Indicates an error when parsing a package from a `&str`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Missing {
    /// The field of name and version is missing.
    NameAndVersion,
    /// The name is missing.
    Name,
    /// The version is missing.
    Version,
}

impl Display for Missing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NameAndVersion => write!(f, "missing name and version field"),
            Self::Name => write!(f, "missing name field"),
            Self::Version => write!(f, "missing version field"),
        }
    }
}

impl Error for Missing {}
