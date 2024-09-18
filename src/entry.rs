use std::cell::LazyCell;
use std::str::FromStr;

use chrono::{DateTime, FixedOffset};
use regex::Regex;

use crate::error::Error;
use crate::message::Message;
use crate::Issuer;

const REGEX_STR: &str = r"\[(.+)\] \[(.+)\] (.+)";
#[allow(clippy::declare_interior_mutable_const)]
const REGEX: LazyCell<Regex> = LazyCell::new(|| Regex::new(REGEX_STR).expect("malformed regex"));
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
        #[allow(clippy::borrow_interior_mutable_const)]
        let (_, [timestamp, issuer, message]) = REGEX
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
