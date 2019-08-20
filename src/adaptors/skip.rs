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

    fn size_hint(&self) -> (usize, Option<usize>) {
        size_hint::sub(self.iter.size_hint(), self.n)
    }

    fn try_nth(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        if self.n == 0 {
            self.iter.try_nth(n)
        } else {
            match self.iter.try_nth(mem::take(&mut self.n) - 1)? {
                Ok(_) => self.iter.try_nth(n),
                Err(_) => Ok(Err(n)),
            }
        }
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

impl<I> ExactSizeTryIterator for Skip<I> where I: ExactSizeTryIterator {}
