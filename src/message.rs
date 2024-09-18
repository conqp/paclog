use std::str::FromStr;

pub use error::Error;
pub use package::Package;
pub use upgrade::Upgrade;

mod error;
mod package;
mod upgrade;

/// Log messages.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Message {
    /// A full system upgrade has been started.
    StartingFullSystemUpgrade,
    /// A transaction has been started.
    TransactionStarted,
    /// A package has been installed.
    Installed(Package),
    /// A package has been upgraded.
    Upgraded(Upgrade),
    /// A package has been reinstalled.
    Reinstalled(Package),
    /// A package has been removed.
    Removed(Package),
    /// A transaction has been completed.
    TransactionCompleted,
    /// Other messages.
    Other(String),
}

impl FromStr for Message {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut parts = text.splitn(2, ' ');

        let Some(command) = parts.next() else {
            return Ok(Self::Other(text.to_string()));
        };

        match command {
            "installed" => Ok(Self::Installed(Package::from_str(
                parts.next().ok_or(Error::MissingParameters)?,
            )?)),
            "upgraded" => Ok(Self::Upgraded(Upgrade::from_str(
                parts.next().ok_or(Error::MissingParameters)?,
            )?)),
            "reinstalled" => Ok(Self::Reinstalled(Package::from_str(
                parts.next().ok_or(Error::MissingParameters)?,
            )?)),
            "removed" => Ok(Self::Removed(Package::from_str(
                parts.next().ok_or(Error::MissingParameters)?,
            )?)),
            _ => match text {
                "starting full system upgrade" => Ok(Self::StartingFullSystemUpgrade),
                "transaction started" => Ok(Self::TransactionStarted),
                "transaction completed" => Ok(Self::TransactionCompleted),
                text => Ok(Self::Other(text.to_string())),
            },
        }
    }
}
