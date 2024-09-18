mod error;

pub use error::Missing;
use std::str::FromStr;

const REGEX: &str = r"^(.+) \((.+)\)$";

/// Represents information about a package.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Package {
    name: String,
    version: String,
}

impl Package {
    /// Return the package's name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the package's version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }
}

impl FromStr for Package {
    type Err = Missing;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let captures = regex::Regex::new(REGEX)
            .unwrap_or_else(|_| unreachable!())
            .captures(text)
            .ok_or(Missing::NameAndVersion)?;
        let mut matches = captures.iter();
        matches.next(); // Skip the full match

        Ok(Self {
            name: matches
                .next()
                .ok_or(Missing::Name)?
                .ok_or(Missing::Name)?
                .as_str()
                .to_string(),
            version: matches
                .next()
                .ok_or(Missing::Version)?
                .ok_or(Missing::Version)?
                .as_str()
                .to_string(),
        })
    }
}
