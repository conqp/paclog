use std::cell::LazyCell;
use std::str::FromStr;

use regex::Regex;

const REGEX_STR: &str = r"^(.+) \((.+)\)$";
#[allow(clippy::declare_interior_mutable_const)]
const REGEX: LazyCell<Regex> = LazyCell::new(|| Regex::new(REGEX_STR).expect("malformed regex"));

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
        #[allow(clippy::borrow_interior_mutable_const)]
        let (_, [name, version]) = REGEX
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
