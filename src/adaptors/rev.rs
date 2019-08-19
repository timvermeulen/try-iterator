use super::*;

pub struct Rev<I> {
    iter: I,
}

impl<I> Rev<I>
where
    I: DoubleEndedTryIterator,
{
    pub(crate) fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I> TryIterator for Rev<I>
where
    I: DoubleEndedTryIterator,
{
    type Item = I::Item;
    type Error = I::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.iter.next_back()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<I> ExactSizeTryIterator for Rev<I> where I: DoubleEndedTryIterator + ExactSizeTryIterator {}
