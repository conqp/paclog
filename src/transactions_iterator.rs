use crate::{Entry, Issuer, Package, Transaction};
use crate::{Message, Upgrade};
use log::warn;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TransactionsIterator<T>
where
    T: Iterator<Item = Entry>,
{
    entries: T,
    start: Option<Entry>,
    completion: Option<Entry>,
    installed: Vec<Package>,
    upgraded: Vec<Upgrade>,
    reinstalled: Vec<Package>,
    removed: Vec<Package>,
    hooks: Vec<Entry>,
}

impl<T> TransactionsIterator<T>
where
    T: Iterator<Item = Entry>,
{
    fn reset(&mut self, start: Entry) {
        self.start.replace(start);
        self.completion = None;
        self.installed.clear();
        self.upgraded.clear();
        self.reinstalled.clear();
        self.removed.clear();
        self.hooks.clear();
    }

    const fn is_within_transaction(&self) -> bool {
        self.start.is_some() && self.completion.is_none()
    }

    fn make_transaction(&mut self) -> Option<Transaction> {
        self.start.take().map(|start| {
            Transaction::new(
                start,
                self.installed.as_slice().into(),
                self.upgraded.as_slice().into(),
                self.reinstalled.as_slice().into(),
                self.removed.as_slice().into(),
                self.completion.take(),
                self.hooks.as_slice().into(),
            )
        })
    }
}

impl<T> From<T> for TransactionsIterator<T>
where
    T: Iterator<Item = Entry>,
{
    fn from(entries: T) -> Self {
        Self {
            entries,
            start: None,
            completion: None,
            installed: Vec::new(),
            upgraded: Vec::new(),
            reinstalled: Vec::new(),
            removed: Vec::new(),
            hooks: Vec::new(),
        }
    }
}

impl<T> Iterator for TransactionsIterator<T>
where
    T: Iterator<Item = Entry>,
{
    type Item = Transaction;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(entry) = self.entries.next() {
                match entry.message() {
                    Message::TransactionStarted => {
                        if let Some(transaction) = self.make_transaction() {
                            self.reset(entry);
                            return Some(transaction);
                        }

                        self.reset(entry);
                    }
                    Message::TransactionCompleted => {
                        self.completion.replace(entry);
                    }
                    Message::Other(_) => {
                        if matches!(entry.issuer(), Issuer::Alpm | Issuer::AlpmScriptlet) {
                            self.hooks.push(entry);
                        }
                    }
                    Message::Installed(package) => {
                        if self.is_within_transaction() {
                            self.installed.push(package.clone());
                        } else {
                            warn!("discarding package install outside of transaction: {package:?}");
                        }
                    }
                    Message::Upgraded(upgrade) => {
                        if self.is_within_transaction() {
                            self.upgraded.push(upgrade.clone());
                        } else {
                            warn!("discarding package upgrade outside of transaction: {upgrade:?}");
                        }
                    }
                    Message::Reinstalled(package) => {
                        if self.is_within_transaction() {
                            self.reinstalled.push(package.clone());
                        } else {
                            warn!(
                                "discarding package reinstall outside of transaction: {package:?}"
                            );
                        }
                    }
                    Message::Removed(package) => {
                        if self.is_within_transaction() {
                            self.removed.push(package.clone());
                        } else {
                            warn!("discarding package removal outside of transaction: {package:?}");
                        }
                    }
                    Message::StartingFullSystemUpgrade => {
                        // TODO: Maybe handle this?
                    }
                }
            } else {
                return self.make_transaction();
            }
        }
    }
}
