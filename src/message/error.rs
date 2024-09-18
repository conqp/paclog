use std::fmt::{Debug, Display};

use super::{package, upgrade};

/// Represents an error while parsing a message.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    /// An error parsing a package occurred.
    Package(package::Error),
    /// An error parsing an upgrade occurred.
    Upgrade(upgrade::Error),
    /// Expected log parameters are missing.
    MissingParameters,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Package(error) => Display::fmt(error, f),
            Self::Upgrade(error) => Display::fmt(error, f),
            Self::MissingParameters => write!(f, "missing parameters"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Package(error) => Some(error),
            Self::Upgrade(error) => Some(error),
            Self::MissingParameters => None,
        }
    }
}

impl From<package::Error> for Error {
    fn from(error: package::Error) -> Self {
        Self::Package(error)
    }
}

impl From<upgrade::Error> for Error {
    fn from(error: upgrade::Error) -> Self {
        Self::Upgrade(error)
    }
}
