use crate::message::{Message, Package};
use crate::{Entry, Upgrade};

/// Representation of a pacman transaction.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Transaction {
    entries: Box<[Entry]>,
}

impl Transaction {
    #[must_use]
    pub(crate) fn new(entries: Box<[Entry]>) -> Self {
        Self { entries }
    }

    /// Return a slice of the entries.
    #[must_use]
    pub const fn entries(&self) -> &[Entry] {
        &self.entries
    }

    /// Return the amount of entries.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.entries.len()
    }

    /// Determine if the transaction is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Return an iterator of the entries.
    pub fn iter(&self) -> impl Iterator<Item = &Entry> {
        self.entries.iter()
    }

    /// Return the start entry.
    #[must_use]
    pub fn begin(&self) -> Option<&Entry> {
        self.entries
            .iter()
            .find(|entry| matches!(entry.message(), Message::TransactionStarted))
    }

    /// Return the end entry.
    #[must_use]
    pub fn end(&self) -> Option<&Entry> {
        self.entries
            .iter()
            .find(|entry| matches!(entry.message(), Message::TransactionCompleted))
    }

    /// Return an iterator of packet names that were installed in this transaction.
    pub fn installed(&self) -> impl Iterator<Item = &Package> {
        self.entries.iter().filter_map(|entry| {
            if let Message::Installed(package) = entry.message() {
                Some(package)
            } else {
                None
            }
        })
    }

    /// Return an iterator of packet names that were upgraded in this transaction.
    pub fn upgraded(&self) -> impl Iterator<Item = &Upgrade> {
        self.entries.iter().filter_map(|entry| {
            if let Message::Upgraded(upgrade) = entry.message() {
                Some(upgrade)
            } else {
                None
            }
        })
    }

    /// Return an iterator of packet names that were reinstalled in this transaction.
    pub fn reinstalled(&self) -> impl Iterator<Item = &Package> {
        self.entries.iter().filter_map(|entry| {
            if let Message::Reinstalled(package) = entry.message() {
                Some(package)
            } else {
                None
            }
        })
    }

    /// Return an iterator of packet names that were removed in this transaction.
    pub fn removed(&self) -> impl Iterator<Item = &Package> {
        self.entries.iter().filter_map(|entry| {
            if let Message::Removed(package) = entry.message() {
                Some(package)
            } else {
                None
            }
        })
    }

    /// Return an iterator of all packages that were part of this transaction.
    pub fn packages(&self) -> impl Iterator<Item = &str> {
        self.entries
            .iter()
            .filter_map(|entry| match entry.message() {
                Message::Installed(package)
                | Message::Reinstalled(package)
                | Message::Removed(package) => Some(package.name()),
                Message::Upgraded(upgrade) => Some(upgrade.name()),
                _ => None,
            })
    }

    /// Return an iterator of all packages that were retained in this transaction.
    pub fn retained(&self) -> impl Iterator<Item = &str> {
        self.entries
            .iter()
            .filter_map(|entry| match entry.message() {
                Message::Installed(package) | Message::Reinstalled(package) => Some(package.name()),
                Message::Upgraded(upgrade) => Some(upgrade.name()),
                _ => None,
            })
    }
}
