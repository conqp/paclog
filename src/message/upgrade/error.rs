use std::error::Error;
use std::fmt::{Display, Formatter};

/// Indicates an error when parsing an upgrade from a `&str`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Missing {
    /// The field of name and version is missing.
    NameAndVersions,
    /// The name is missing.
    Name,
    /// The old version is missing.
    OldVersion,
    /// The new version is missing.
    NewVersion,
}

impl Display for Missing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NameAndVersions => write!(f, "missing name and versions field"),
            Self::Name => write!(f, "missing name field"),
            Self::OldVersion => write!(f, "missing old version field"),
            Self::NewVersion => write!(f, "missing new version field"),
        }
    }
}

impl Error for Missing {}
