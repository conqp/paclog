use std::fmt::Display;

use chrono::ParseError;

/// Log entry parsing error.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    /// Timestamp is missing.
    MissingTimestamp,
    /// Issuer is missing.
    MissingIssuer,
    /// Timestamp has invalid format.
    InvalidTimestamp(ParseError),
    /// Message is invalid.
    InvalidMessage(crate::message::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingTimestamp => write!(f, "missing timestamp"),
            Self::MissingIssuer => write!(f, "missing issuer"),
            Self::InvalidTimestamp(error) => write!(f, "invalid timestamp: {error}"),
            Self::InvalidMessage(error) => write!(f, "invalid message: {error}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidTimestamp(error) => Some(error),
            Self::InvalidMessage(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        Self::InvalidTimestamp(error)
    }
}

impl From<crate::message::Error> for Error {
    fn from(error: crate::message::Error) -> Self {
        Self::InvalidMessage(error)
    }
}
