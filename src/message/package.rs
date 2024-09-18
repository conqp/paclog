use std::str::FromStr;

use regex::Regex;

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
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let (_, [name, version]) = Regex::new(REGEX)
            .unwrap_or_else(|_| unreachable!())
            .captures_iter(text)
            .map(|capture| capture.extract())
            .next()
            .ok_or_else(|| text.to_string())?;

        Ok(Self {
            name: name.to_string(),
            version: version.to_string(),
        })
    }
}
