use std::cell::LazyCell;
use std::str::FromStr;

use regex::Regex;

const REGEX_STR: &str = r"^(.+) \((.+) -> (.+)\)$";
#[allow(clippy::declare_interior_mutable_const)]
const REGEX: LazyCell<Regex> = LazyCell::new(|| Regex::new(REGEX_STR).expect("malformed regex"));

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
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        #[allow(clippy::borrow_interior_mutable_const)]
        let (_, [name, old_version, new_version]) = REGEX
            .captures_iter(text)
            .map(|capture| capture.extract())
            .next()
            .ok_or_else(|| text.to_string())?;

        Ok(Self {
            name: name.to_string(),
            old_version: old_version.to_string(),
            new_version: new_version.to_string(),
        })
    }
}
