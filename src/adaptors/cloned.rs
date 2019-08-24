use super::*;

#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Cloned<I> {
    iter: I,
}

impl<'a, I> Cloned<I> {
    pub(crate) fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<'a, I, T> TryIterator for Cloned<I>
where
    I: TryIterator<Item = &'a T>,
    T: Clone + 'a,
{
    type Item = T;
    type Error = I::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.find(|_| true)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn try_fold<Acc, F, R>(&mut self, acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        self.iter.try_fold(acc, |acc, x| f(acc, x.clone()))
    }
}

impl<'a, I, T> DoubleEndedTryIterator for Cloned<I>
where
    I: DoubleEndedTryIterator<Item = &'a T>,
    T: Clone + 'a,
{
    fn next_back(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.rfind(|_| true)
    }

    fn try_rfold<Acc, F, R>(&mut self, acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        self.iter.try_rfold(acc, |acc, x| f(acc, x.clone()))
    }
}

impl<'a, I, T> ExactSizeTryIterator for Cloned<I>
where
    I: ExactSizeTryIterator<Item = &'a T>,
    T: Clone + 'a,
{
}

impl<'a, I, T> FusedTryIterator for Cloned<I>
where
    I: FusedTryIterator<Item = &'a T>,
    T: Clone + 'a,
{
}
