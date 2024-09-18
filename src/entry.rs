use std::str::FromStr;

use chrono::{DateTime, FixedOffset};
use regex::Regex;

use crate::error::Error;
use crate::message::Message;
use crate::Issuer;

const REGEX: &str = r"\[(.+)\] \[(.+)\] (.+)";
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
        let (_, [timestamp, issuer, message]) = Regex::new(REGEX)
            .unwrap_or_else(|_| unreachable!())
            .captures_iter(s)
            .map(|capture| capture.extract())
            .next()
            .ok_or_else(|| Error::MalformedEntry(s.to_string()))?;

        Ok(Self {
            timestamp: DateTime::parse_from_str(timestamp, TIME_FORMAT)?,
            issuer: issuer.to_string().into(),
            message: Message::from_str(message)?,
        })
    }
}
