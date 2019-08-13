use super::*;

pub struct Inspect<I, F> {
    iter: I,
    f: F,
}

impl<I, F, R> Inspect<I, F>
where
    I: TryIterator,
    F: FnMut(&I::Item) -> R,
    R: Try<Ok = (), Error = I::Error>,
{
    pub(crate) fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<I, F, R> TryIterator for Inspect<I, F>
where
    I: TryIterator,
    F: FnMut(&I::Item) -> R,
    R: Try<Ok = (), Error = I::Error>,
{
    type Item = I::Item;
    type Error = I::Error;

    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        self.find(|_| true)
    }

    fn try_fold<Acc, G, Q>(&mut self, acc: Acc, mut g: G) -> Q
    where
        G: FnMut(Acc, Self::Item) -> Q,
        Q: Try<Ok = Acc>,
        Q::Error: From<Self::Error>,
    {
        let f = &mut self.f;
        self.iter.try_fold(acc, |acc, x| {
            f(&x)?;
            g(acc, x)
        })
    }
}
