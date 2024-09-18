use crate::Entry;
use crate::TransactionsIterator;

/// Group entries into transactions.
pub trait Transactions: Iterator<Item = Entry> + Sized {
    /// Return an iterator of [`Transaction`]s.
    fn transactions(self) -> TransactionsIterator<Self>;
}

impl<T> Transactions for T
where
    T: Iterator<Item = Entry>,
{
    fn transactions(self) -> TransactionsIterator<Self> {
        TransactionsIterator::from(self)
    }
}
