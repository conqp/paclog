use crate::message::Message;
use crate::Entry;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Transaction {
    entries: Box<[Entry]>,
}

impl Transaction {
    #[must_use]
    pub(crate) fn new(entries: Box<[Entry]>) -> Self {
        Self { entries }
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.entries.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Entry> {
        self.entries.iter()
    }

    #[must_use]
    pub fn begin(&self) -> Option<&Entry> {
        self.entries
            .iter()
            .find(|entry| matches!(entry.message(), Message::TransactionStarted))
    }

    #[must_use]
    pub fn end(&self) -> Option<&Entry> {
        self.entries
            .iter()
            .find(|entry| matches!(entry.message(), Message::TransactionCompleted))
    }

    pub fn installed(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().filter_map(|entry| {
            if let Message::Installed { package, .. } = entry.message() {
                Some(package.as_str())
            } else {
                None
            }
        })
    }

    pub fn upgraded(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().filter_map(|entry| {
            if let Message::Upgraded { package, .. } = entry.message() {
                Some(package.as_str())
            } else {
                None
            }
        })
    }

    pub fn reinstalled(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().filter_map(|entry| {
            if let Message::Reinstalled { package, .. } = entry.message() {
                Some(package.as_str())
            } else {
                None
            }
        })
    }

    pub fn packages(&self) -> impl Iterator<Item = &str> {
        self.entries
            .iter()
            .filter_map(|entry| match entry.message() {
                Message::Installed { package, .. }
                | Message::Upgraded { package, .. }
                | Message::Reinstalled { package, .. } => Some(package.as_str()),
                _ => None,
            })
    }
}

impl From<Box<[Entry]>> for Transaction {
    fn from(entries: Box<[Entry]>) -> Self {
        Self { entries }
    }
}
