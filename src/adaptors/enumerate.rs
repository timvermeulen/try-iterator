use super::*;

#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Enumerate<I> {
    iter: I,
    count: usize,
}

impl<I> Enumerate<I> {
    pub(crate) fn new(iter: I) -> Self {
        Self { iter, count: 0 }
    }
}

impl<I> TryIterator for Enumerate<I>
where
    I: TryIterator,
{
    type Item = (usize, I::Item);
    type Error = I::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.find(|_| true)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn try_nth(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        Ok(self.iter.try_nth(n)?.map(|x| {
            let i = self.count + n;
            self.count = i + 1;
            (i, x)
        }))
    }

    fn try_fold<Acc, F, R>(&mut self, acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        let count = &mut self.count;
        self.iter.try_fold(acc, |acc, x| {
            let c = *count;
            *count += 1;
            f(acc, (c, x))
        })
    }

    fn count(self) -> Result<usize, Self::Error> {
        self.iter.count()
    }
}

impl<I> DoubleEndedTryIterator for Enumerate<I>
where
    I: DoubleEndedTryIterator + ExactSizeTryIterator,
{
    fn next_back(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.rfind(|_| true)
    }

    fn try_nth_back(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        Ok(self
            .iter
            .try_nth_back(n)?
            .map(|x| (self.count + self.len(), x)))
    }

    fn try_rfold<Acc, F, R>(&mut self, acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        let mut count = self.count + self.iter.len();
        self.iter.try_rfold(acc, move |acc, item| {
            count -= 1;
            f(acc, (count, item))
        })
    }
}

impl<I> ExactSizeTryIterator for Enumerate<I> where I: ExactSizeTryIterator {}

impl<I> FusedTryIterator for Enumerate<I> where I: FusedTryIterator {}
