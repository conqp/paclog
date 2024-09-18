use std::fmt::{Debug, Display};

/// Represents an error while parsing a message.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    /// The message was malformed.
    MalformedMessage(String),
    /// Expected log parameters are missing.
    MissingParameters,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MalformedMessage(message) => write!(f, "malformed message: {message}"),
            Self::MissingParameters => write!(f, "missing parameters"),
        }
    }
}

impl std::error::Error for Error {}

impl From<String> for Error {
    fn from(message: String) -> Self {
        Self::MalformedMessage(message)
    }
}
