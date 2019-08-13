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

    fn try_fold<Acc, G, Q>(&mut self, acc: Acc, mut g: G) -> Q
    where
        G: FnMut(Acc, Self::Item) -> Q,
        Q: Try<Ok = Acc>,
        Q::Error: From<Self::Error>,
    {
        let count = &mut self.count;
        self.iter.try_fold(acc, |acc, x| {
            let c = *count;
            *count += 1;
            g(acc, (c, x))
        })
    }
}
