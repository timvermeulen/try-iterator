use super::*;

pub struct IteratorWrapper<I, E> {
    iter: I,
    _marker: PhantomData<E>,
}

impl<I, E> IteratorWrapper<I, E>
where
    I: Iterator,
{
    pub(crate) fn new(iter: I) -> Self {
        Self {
            iter,
            _marker: PhantomData,
        }
    }
}

impl<I, E> Clone for IteratorWrapper<I, E>
where
    I: Iterator + Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.iter.clone())
    }
}

impl<I, E> Debug for IteratorWrapper<I, E>
where
    I: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("IteratorWrapper")
            .field("iter", &self.iter)
            .finish()
    }
}

impl<I, E> TryIterator for IteratorWrapper<I, E>
where
    I: Iterator,
{
    type Item = I::Item;
    type Error = E;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        Ok(self.iter.next())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn nth(&mut self, n: usize) -> Result<Option<Self::Item>, Self::Error> {
        Ok(self.iter.nth(n))
    }

    fn try_fold<Acc, F, R>(&mut self, acc: Acc, f: F) -> R
    where
        Self: Sized,
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        self.iter.try_fold(acc, f)
    }
}

impl<I, E> DoubleEndedTryIterator for IteratorWrapper<I, E>
where
    I: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        Ok(self.iter.next_back())
    }

    fn nth_back(&mut self, n: usize) -> Result<Option<Self::Item>, Self::Error> {
        Ok(self.iter.nth_back(n))
    }

    fn try_rfold<Acc, F, R>(&mut self, acc: Acc, f: F) -> R
    where
        Self: Sized,
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        self.iter.try_rfold(acc, f)
    }
}

impl<I, E> ExactSizeTryIterator for IteratorWrapper<I, E> where I: ExactSizeIterator {}

impl<I, E> FusedTryIterator for IteratorWrapper<I, E> where I: FusedIterator {}
