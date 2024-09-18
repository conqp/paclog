mod error;

pub use error::MissingField;
use std::str::FromStr;

const INSTALL_REGEX: &str = r"^(.+) \((.+)\)$";
const UPGRADE_REGEX: &str = r"^(.+) \((.+) -> (.+)\)$";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Message {
    StartingFullSystemUpgrade,
    TransactionStarted,
    Installed {
        package: String,
        version: String,
    },
    Upgraded {
        package: String,
        old_version: String,
        new_version: String,
    },
    Reinstalled {
        package: String,
        version: String,
    },
    TransactionCompleted,
    Raw(String),
}

impl FromStr for Message {
    type Err = MissingField;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut parts = text.splitn(2, ' ');

        let Some(command) = parts.next() else {
            return Ok(Self::Raw(text.to_string()));
        };

        match command {
            "installed" => {
                let package_and_version = parts.next().ok_or(MissingField::PackageAndVersion)?;
                let captures = regex::Regex::new(INSTALL_REGEX)
                    .unwrap_or_else(|_| unreachable!())
                    .captures(package_and_version)
                    .ok_or(MissingField::PackageAndVersion)?;
                let mut matches = captures.iter();
                matches.next(); // Skip the full match

                Ok(Self::Installed {
                    package: matches
                        .next()
                        .ok_or(MissingField::Package)?
                        .ok_or(MissingField::Package)?
                        .as_str()
                        .to_string(),
                    version: matches
                        .next()
                        .ok_or(MissingField::Version)?
                        .ok_or(MissingField::Version)?
                        .as_str()
                        .to_string(),
                })
            }
            "upgraded" => {
                let package_and_version = parts.next().ok_or(MissingField::PackageAndVersion)?;
                let captures = regex::Regex::new(UPGRADE_REGEX)
                    .unwrap_or_else(|_| unreachable!())
                    .captures(package_and_version)
                    .ok_or(MissingField::PackageAndVersion)?;
                let mut matches = captures.iter();
                matches.next(); // Skip the full match

                Ok(Self::Upgraded {
                    package: matches
                        .next()
                        .ok_or(MissingField::Package)?
                        .ok_or(MissingField::Package)?
                        .as_str()
                        .to_string(),
                    old_version: matches
                        .next()
                        .ok_or(MissingField::Version)?
                        .ok_or(MissingField::Version)?
                        .as_str()
                        .to_string(),
                    new_version: matches
                        .next()
                        .ok_or(MissingField::Version)?
                        .ok_or(MissingField::Version)?
                        .as_str()
                        .to_string(),
                })
            }
            "reinstalled" => {
                let package_and_version = parts.next().ok_or(MissingField::PackageAndVersion)?;
                let captures = regex::Regex::new(INSTALL_REGEX)
                    .unwrap_or_else(|_| unreachable!())
                    .captures(package_and_version)
                    .ok_or(MissingField::PackageAndVersion)?;
                let mut matches = captures.iter();
                matches.next(); // Skip the full match

                Ok(Self::Reinstalled {
                    package: matches
                        .next()
                        .ok_or(MissingField::Package)?
                        .ok_or(MissingField::Package)?
                        .as_str()
                        .to_string(),
                    version: matches
                        .next()
                        .ok_or(MissingField::Version)?
                        .ok_or(MissingField::Version)?
                        .as_str()
                        .to_string(),
                })
            }
            _ => match text {
                "starting full system upgrade" => Ok(Self::StartingFullSystemUpgrade),
                "transaction started" => Ok(Self::TransactionStarted),
                "transaction completed" => Ok(Self::TransactionCompleted),
                text => Ok(Self::Raw(text.to_string())),
            },
        }
    }
}
