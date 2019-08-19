use super::*;

pub trait IntoTryIterator {
    type Item;
    type Error;
    type IntoTryIter: TryIterator<Item = Self::Item, Error = Self::Error>;

    fn into_try_iter(self) -> Self::IntoTryIter;
}

impl<I> IntoTryIterator for I
where
    I: TryIterator,
{
    type Item = <Self as TryIterator>::Item;
    type Error = <Self as TryIterator>::Error;
    type IntoTryIter = Self;

    fn into_try_iter(self) -> Self::IntoTryIter {
        self
    }
}
