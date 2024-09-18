use chrono::{DateTime, FixedOffset};

use crate::message::Package;
use crate::{Entry, Upgrade};

/// Representation of a pacman transaction.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Transaction {
    start: Entry,
    installed: Box<[Package]>,
    upgraded: Box<[Upgrade]>,
    reinstalled: Box<[Package]>,
    removed: Box<[Package]>,
    completion: Option<Entry>,
    hooks: Box<[Entry]>,
}

impl Transaction {
    /// Create a new transaction.
    ///
    /// This method is crate-only since we do not want users to
    /// create new transactions from arbitrary entries.
    #[must_use]

    pub(crate) const fn new(
        start: Entry,
        installed: Box<[Package]>,
        upgraded: Box<[Upgrade]>,
        reinstalled: Box<[Package]>,
        removed: Box<[Package]>,
        completion: Option<Entry>,
        hooks: Box<[Entry]>,
    ) -> Self {
        Self {
            start,
            installed,
            upgraded,
            reinstalled,
            removed,
            completion,
            hooks,
        }
    }

    /// Return the start entry.
    #[must_use]
    pub const fn start(&self) -> &Entry {
        &self.start
    }

    /// Return a slice of packets that were installed in this transaction.
    #[must_use]
    pub const fn installed(&self) -> &[Package] {
        &self.installed
    }

    /// Return a slice of packets that were upgraded in this transaction.
    #[must_use]
    pub const fn upgraded(&self) -> &[Upgrade] {
        &self.upgraded
    }

    /// Return a slice of packets that were reinstalled in this transaction.
    #[must_use]
    pub const fn reinstalled(&self) -> &[Package] {
        &self.reinstalled
    }

    /// Return a slice of packets that were removed in this transaction.
    #[must_use]
    pub const fn removed(&self) -> &[Package] {
        &self.removed
    }

    /// Return the completion entry.
    #[must_use]
    pub const fn completion(&self) -> Option<&Entry> {
        self.completion.as_ref()
    }

    /// Return the start time.
    #[must_use]
    pub const fn begin(&self) -> DateTime<FixedOffset> {
        self.start.timestamp()
    }

    /// Return the end time.
    #[must_use]
    pub fn end(&self) -> Option<DateTime<FixedOffset>> {
        self.completion().map(Entry::timestamp)
    }

    /// Return an iterator of all packages that were part of this transaction.
    pub fn packages(&self) -> impl Iterator<Item = &str> {
        self.installed
            .iter()
            .chain(self.reinstalled.iter())
            .chain(self.removed.iter())
            .map(Package::name)
            .chain(self.upgraded.iter().map(Upgrade::name))
    }

    /// Return an iterator of all packages that were retained in this transaction.
    pub fn retained(&self) -> impl Iterator<Item = &str> {
        self.installed
            .iter()
            .chain(self.reinstalled.iter())
            .map(Package::name)
            .chain(self.upgraded.iter().map(Upgrade::name))
    }
}
