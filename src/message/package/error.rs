use std::fmt::{Display, Formatter};

/// Indicates an error when parsing a package from a `&str`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    /// The package entry is malformed.
    MalformedPackage(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MalformedPackage(text) => write!(f, "malformed package: {text}"),
        }
    }
}

impl std::error::Error for Error {}
