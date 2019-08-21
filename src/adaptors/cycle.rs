use super::*;

#[derive(Clone, Debug)]
pub struct Cycle<I> {
    iter: I,
    current: I,
}

impl<I> Cycle<I>
where
    I: Clone,
{
    pub(crate) fn new(iter: I) -> Self {
        let current = iter.clone();
        Self { iter, current }
    }
}

impl<I> TryIterator for Cycle<I>
where
    I: TryIterator + Clone,
{
    type Item = I::Item;
    type Error = I::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.find(|_| true)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.iter.size_hint() {
            size_hint::ZERO => size_hint::ZERO,
            (0, _) => (0, None),
            _ => (usize::max_value(), None),
        }
    }

    fn try_nth(&mut self, mut n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        loop {
            n = match self.current.try_nth(n)? {
                Ok(x) => return Ok(Ok(x)),
                Err(n) => n,
            };
            self.current = self.iter.clone();
        }
    }

    fn try_fold<Acc, F, R>(&mut self, mut acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        // fully iterate the current iterator. this is necessary because
        // `self.iter` may be empty even when `self.orig` isn't
        acc = self.current.try_fold(acc, &mut f)?;
        self.current = self.iter.clone();

        // complete a full cycle, keeping track of whether the cycled
        // iterator is empty or not. we need to return early in case
        // of an empty iterator to prevent an infinite loop
        let mut is_empty = true;
        acc = self.current.try_fold(acc, |acc, x| {
            is_empty = false;
            f(acc, x)
        })?;

        if is_empty {
            return Try::from_ok(acc);
        }

        loop {
            self.iter = self.iter.clone();
            acc = self.current.try_fold(acc, &mut f)?;
        }
    }
}

impl<I> FusedTryIterator for Cycle<I> where I: TryIterator + Clone {}
