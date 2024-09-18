use crate::Entry;
use crate::TransactionsIterator;

pub trait Transactions: Iterator<Item = Entry> + Sized {
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
