use super::*;

#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Copied<I> {
    iter: I,
}

impl<'a, I> Copied<I> {
    pub(crate) fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<'a, I, T> TryIterator for Copied<I>
where
    I: TryIterator<Item = &'a T>,
    T: Copy + 'a,
{
    type Item = T;
    type Error = I::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.find(|_| true)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn try_nth(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        try { self.iter.try_nth(n)?.map(|&x| x) }
    }

    fn try_fold<Acc, F, R>(&mut self, acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        self.iter.try_fold(acc, |acc, &x| f(acc, x))
    }

    fn count(self) -> Result<usize, Self::Error> {
        self.iter.count()
    }

    fn last(self) -> Result<Option<Self::Item>, Self::Error> {
        try { self.iter.last()?.map(|&x| x) }
    }
}

impl<'a, I, T> DoubleEndedTryIterator for Copied<I>
where
    I: DoubleEndedTryIterator<Item = &'a T>,
    T: Copy + 'a,
{
    fn next_back(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.rfind(|_| true)
    }

    fn try_nth_back(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        try { self.iter.try_nth_back(n)?.map(|&x| x) }
    }

    fn try_rfold<Acc, F, R>(&mut self, acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        self.iter.try_rfold(acc, |acc, &x| f(acc, x))
    }
}

impl<'a, I, T> ExactSizeTryIterator for Copied<I>
where
    I: ExactSizeTryIterator<Item = &'a T>,
    T: Copy + 'a,
{
}

impl<'a, I, T> FusedTryIterator for Copied<I>
where
    I: FusedTryIterator<Item = &'a T>,
    T: Copy + 'a,
{
}
