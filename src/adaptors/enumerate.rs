use super::*;

pub struct Enumerate<I> {
    iter: I,
    count: usize,
}

impl<I> Enumerate<I>
where
    I: TryIterator,
{
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
}

impl<I> ExactSizeTryIterator for Enumerate<I> where I: ExactSizeTryIterator {}
