use super::*;

pub struct Skip<I> {
    iter: I,
    n: usize,
}

impl<I> Skip<I>
where
    I: TryIterator,
{
    pub(crate) fn new(iter: I, n: usize) -> Self {
        Self { iter, n }
    }
}

impl<I> TryIterator for Skip<I>
where
    I: TryIterator,
{
    type Item = I::Item;
    type Error = I::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.find(|_| true)
    }

    fn try_fold<Acc, F, R>(&mut self, acc: Acc, f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        let n = self.n;
        self.n = 0;
        if n > 0 && self.iter.nth(n - 1)?.is_none() {
            Try::from_ok(acc)
        } else {
            self.iter.try_fold(acc, f)
        }
    }
}
