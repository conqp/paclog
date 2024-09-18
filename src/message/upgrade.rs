use std::str::FromStr;

pub use error::Missing;

mod error;

const REGEX: &str = r"^(.+) \((.+) -> (.+)\)$";

/// Represents a package upgrade.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Upgrade {
    name: String,
    old_version: String,
    new_version: String,
}

impl Upgrade {
    /// Return the name of the upgraded package.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the old version of the package.
    #[must_use]
    pub fn old_version(&self) -> &str {
        &self.old_version
    }

    /// Return the new version of the package.
    #[must_use]
    pub fn new_version(&self) -> &str {
        &self.new_version
    }
}

impl FromStr for Upgrade {
    type Err = Missing;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let captures = regex::Regex::new(REGEX)
            .unwrap_or_else(|_| unreachable!())
            .captures(text)
            .ok_or(Missing::NameAndVersions)?;
        let mut matches = captures.iter();
        matches.next(); // Skip the full match

        Ok(Self {
            name: matches
                .next()
                .ok_or(Missing::Name)?
                .ok_or(Missing::Name)?
                .as_str()
                .to_string(),
            old_version: matches
                .next()
                .ok_or(Missing::OldVersion)?
                .ok_or(Missing::OldVersion)?
                .as_str()
                .to_string(),
            new_version: matches
                .next()
                .ok_or(Missing::NewVersion)?
                .ok_or(Missing::NewVersion)?
                .as_str()
                .to_string(),
        })
    }
}
