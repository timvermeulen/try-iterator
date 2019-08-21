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

    fn try_nth(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        self.iter.try_nth_back(n)
    }

    fn try_fold<Acc, F, R>(&mut self, acc: Acc, f: F) -> R
    where
        Self: Sized,
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        self.iter.try_rfold(acc, f)
    }
}

impl<I> DoubleEndedTryIterator for Rev<I>
where
    I: DoubleEndedTryIterator,
{
    fn next_back(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.iter.next()
    }

    fn try_nth_back(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        self.iter.try_nth(n)
    }

    fn try_rfold<Acc, F, R>(&mut self, acc: Acc, f: F) -> R
    where
        Self: Sized,
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        self.iter.try_fold(acc, f)
    }
}

impl<I> ExactSizeTryIterator for Rev<I> where I: DoubleEndedTryIterator + ExactSizeTryIterator {}

impl<I> FusedTryIterator for Rev<I> where I: DoubleEndedTryIterator + FusedTryIterator {}
