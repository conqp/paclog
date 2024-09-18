use crate::Message;
use crate::{Entry, Transaction};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TransactionsIterator<T>
where
    T: Iterator<Item = Entry>,
{
    entries: T,
    transaction: Vec<Entry>,
    transaction_started: bool,
}

impl<T> From<T> for TransactionsIterator<T>
where
    T: Iterator<Item = Entry>,
{
    fn from(entries: T) -> Self {
        Self {
            entries,
            transaction: Vec::new(),
            transaction_started: false,
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
            let entry = self.entries.next()?;

            if matches!(entry.message(), Message::TransactionStarted) {
                self.transaction.clear();
                self.transaction.push(entry);
                self.transaction_started = true;
                continue;
            };

            if matches!(entry.message(), Message::TransactionCompleted) {
                self.transaction.push(entry);
                self.transaction_started = false;
                return Some(Transaction::new(
                    self.transaction.clone().into_boxed_slice(),
                ));
            };

            if self.transaction_started {
                self.transaction.push(entry);
            }
        }
    }
}
