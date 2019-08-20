use super::*;

pub struct Cloned<I> {
    iter: I,
}

impl<'a, I, T> Cloned<I>
where
    I: TryIterator<Item = &'a T>,
    T: Clone + 'a,
{
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

    fn try_nth(&mut self, n: usize) -> Result<Result<Self::Item, usize>, Self::Error> {
        try { self.iter.try_nth(n)?.map(|x| x.clone()) }
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

impl<'a, I, T> ExactSizeTryIterator for Cloned<I>
where
    I: ExactSizeTryIterator<Item = &'a T>,
    T: Clone + 'a,
{
}