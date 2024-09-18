use chrono::ParseError;
use std::fmt::Display;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    MissingTimestamp,
    MissingIssuer,
    InvalidTimestamp(ParseError),
    InvalidMessage(crate::message::MissingField),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

impl From<crate::message::MissingField> for Error {
    fn from(error: crate::message::MissingField) -> Self {
        Self::InvalidMessage(error)
    }
}
