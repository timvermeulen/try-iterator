use super::*;

pub struct Copied<I> {
    iter: I,
}

impl<'a, I, T> Copied<I>
where
    I: TryIterator<Item = &'a T>,
    T: Copy + 'a,
{
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

    fn try_fold<Acc, F, R>(&mut self, acc: Acc, mut f: F) -> R
    where
        F: FnMut(Acc, Self::Item) -> R,
        R: Try<Ok = Acc>,
        R::Error: From<Self::Error>,
    {
        self.iter.try_fold(acc, |acc, &x| f(acc, x))
    }
}
