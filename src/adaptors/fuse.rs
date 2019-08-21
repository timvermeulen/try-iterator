use super::*;

#[derive(Clone, Debug)]
pub struct Fuse<I> {
    iter: I,
    done: bool,
}

impl<I> Fuse<I> {
    pub(crate) fn new(iter: I) -> Fuse<I> {
        Fuse { iter, done: false }
    }
}

impl<I> TryIterator for Fuse<I>
where
    I: TryIterator,
{
    type Item = I::Item;
    type Error = I::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.find(|_| true)
    }

    default fn size_hint(&self) -> (usize, Option<usize>) {
        if self.done {
            size_hint::ZERO
        } else {
            self.iter.size_hint()
        }
    }

    default fn try_nth(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        if self.done {
            Ok(Err(n))
        } else {
            self.iter.try_nth(n)
        }
    }

    default fn try_fold<Acc, F, R>(&mut self, acc: Acc, f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        Try::from_ok(if self.done {
            acc
        } else {
            let acc = self.iter.try_fold(acc, f)?;
            self.done = true;
            acc
        })
    }
}

impl<I> TryIterator for Fuse<I>
where
    I: FusedTryIterator,
{
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn try_nth(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        self.iter.try_nth(n)
    }

    fn try_fold<Acc, F, R>(&mut self, acc: Acc, f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        self.iter.try_fold(acc, f)
    }
}

impl<I> DoubleEndedTryIterator for Fuse<I>
where
    I: DoubleEndedTryIterator,
{
    fn next_back(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.rfind(|_| true)
    }

    default fn try_nth_back(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        if self.done {
            Ok(Err(n))
        } else {
            self.iter.try_nth_back(n)
        }
    }

    default fn try_rfold<Acc, F, R>(&mut self, acc: Acc, f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        Try::from_ok(if self.done {
            acc
        } else {
            let acc = self.iter.try_rfold(acc, f)?;
            self.done = true;
            acc
        })
    }
}

impl<I> DoubleEndedTryIterator for Fuse<I>
where
    I: DoubleEndedTryIterator + FusedTryIterator,
{
    fn try_nth_back(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        self.iter.try_nth_back(n)
    }

    fn try_rfold<Acc, F, R>(&mut self, acc: Acc, f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        self.iter.try_rfold(acc, f)
    }
}

impl<I> ExactSizeTryIterator for Fuse<I> where I: ExactSizeTryIterator {}

impl<I> FusedTryIterator for Fuse<I> where I: TryIterator {}
