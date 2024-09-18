use crate::error::Error;
use crate::message::Message;
use crate::Issuer;
use chrono::{DateTime, FixedOffset};
use std::str::FromStr;

const TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%z";

/// A log file entry.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Entry {
    timestamp: DateTime<FixedOffset>,
    issuer: Issuer,
    message: Message,
}

impl Entry {
    /// Returns the entry's timestamp.
    #[must_use]
    pub const fn timestamp(&self) -> DateTime<FixedOffset> {
        self.timestamp
    }

    /// Returns the entry's issuer.
    #[must_use]
    pub const fn issuer(&self) -> &Issuer {
        &self.issuer
    }

    /// Returns the entry's message.
    #[must_use]
    pub const fn message(&self) -> &Message {
        &self.message
    }
}

impl FromStr for Entry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(3, ' ');

        Ok(Self {
            timestamp: DateTime::parse_from_str(
                parts
                    .next()
                    .ok_or(Error::MissingTimestamp)?
                    .trim_start_matches('[')
                    .trim_end_matches(']'),
                TIME_FORMAT,
            )?,
            issuer: parts
                .next()
                .ok_or(Error::MissingIssuer)?
                .trim_start_matches('[')
                .trim_end_matches(']')
                .to_string()
                .into(),
            message: Message::from_str(parts.next().ok_or(Error::MissingTimestamp)?)?,
        })
    }
}
