use std::fmt::{Display, Formatter};

/// Indicates an error when parsing an upgrade from a `&str`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    /// The upgrade entry is malformed.
    MalformedUpgrade(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MalformedUpgrade(text) => write!(f, "malformed upgrade: {text}"),
        }
    }
}

impl std::error::Error for Error {}
